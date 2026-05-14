use midnight_notes::core::backlinks::BacklinkService;
use midnight_notes::core::export::ExportService;
use midnight_notes::core::history::HistoryService;
use midnight_notes::core::markdown;
use midnight_notes::core::note::NoteService;
use midnight_notes::core::search::SearchService;
use midnight_notes::core::tag::TagService;
use midnight_notes::storage::Database;

fn setup_db() -> Database {
    Database::open_in_memory().unwrap()
}

#[test]
fn test_note_lifecycle() {
    let db = setup_db();
    let svc = NoteService::new(&db);

    // Create
    let note = svc.create("Test Note", "Hello world").unwrap();
    assert_eq!(note.title, "Test Note");
    assert_eq!(note.content, "Hello world");
    assert!(!note.is_pinned);

    // Read
    let fetched = svc.get(&note.id).unwrap().unwrap();
    assert_eq!(fetched.title, "Test Note");

    // Update
    svc.update(&note.id, "Updated", "New content").unwrap();
    let updated = svc.get(&note.id).unwrap().unwrap();
    assert_eq!(updated.title, "Updated");

    // Pin
    let pinned = svc.toggle_pin(&note.id).unwrap();
    assert!(pinned);
    assert!(svc.get(&note.id).unwrap().unwrap().is_pinned);

    // Archive
    let archived = svc.toggle_archive(&note.id).unwrap();
    assert!(archived);

    // Active list excludes archived
    let active = svc.list_active().unwrap();
    assert!(!active.iter().any(|n| n.id == note.id));

    // Trash
    svc.trash(&note.id).unwrap();
    let trashed = svc.list_trashed().unwrap();
    assert!(trashed.iter().any(|n| n.id == note.id));

    // Permanent delete
    svc.delete(&note.id).unwrap();
    assert!(svc.get(&note.id).unwrap().is_none());
}

#[test]
fn test_tags_and_notes() {
    let db = setup_db();
    let note_svc = NoteService::new(&db);
    let tag_svc = TagService::new(&db);

    let note = note_svc.create("Tagged Note", "Content").unwrap();
    let tag_a = tag_svc.create("work", None, None).unwrap();
    let tag_b = tag_svc.create("personal", None, None).unwrap();

    // Assign tags
    tag_svc.assign_to_note(&tag_a.id, &note.id).unwrap();
    tag_svc.assign_to_note(&tag_b.id, &note.id).unwrap();

    // Get tags for note
    let tags = tag_svc.get_tags_for_note(&note.id).unwrap();
    assert_eq!(tags.len(), 2);

    // Get notes for tag
    let note_ids = tag_svc.get_notes_for_tag(&tag_a.id).unwrap();
    assert!(note_ids.contains(&note.id));

    // Remove tag
    tag_svc.remove_from_note(&tag_b.id, &note.id).unwrap();
    let tags = tag_svc.get_tags_for_note(&note.id).unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].name, "work");
}

#[test]
fn test_backlinks_graph() {
    let db = setup_db();
    let note_svc = NoteService::new(&db);
    let bl_svc = BacklinkService::new(&db);

    let target = note_svc.create("Design Doc", "Content").unwrap();
    let source = note_svc
        .create("Meeting Notes", "See [[Design Doc]] for details.")
        .unwrap();

    bl_svc.refresh(&source.id, &source.content).unwrap();

    // Linked mentions
    let mentions = bl_svc.get_linked_mentions(&target.id).unwrap();
    assert_eq!(mentions.len(), 1);
    assert_eq!(mentions[0].id, source.id);

    // Outgoing links
    let outgoing = bl_svc.get_outgoing_links(&source.id).unwrap();
    assert_eq!(outgoing.len(), 1);
    assert_eq!(outgoing[0].id, target.id);

    // Update source to remove link — backlink should be cleared
    bl_svc.refresh(&source.id, "No links here.").unwrap();
    let mentions = bl_svc.get_linked_mentions(&target.id).unwrap();
    assert_eq!(mentions.len(), 0);
}

#[test]
fn test_fts5_search() {
    let db = setup_db();
    let note_svc = NoteService::new(&db);
    let search_svc = SearchService::new(&db);

    note_svc
        .create("Rust Tutorial", "Learn about ownership")
        .unwrap();
    note_svc
        .create("Python Guide", "Learn about decorators")
        .unwrap();

    let results = search_svc.search("Rust").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Rust Tutorial");

    // Smart views
    search_svc.save_smart_view("Rust Notes", "Rust").unwrap();
    let view_results = search_svc.execute_smart_view("Rust Notes").unwrap();
    assert_eq!(view_results.len(), 1);
}

#[test]
fn test_version_history() {
    let db = setup_db();
    let note_svc = NoteService::new(&db);
    let hist_svc = HistoryService::new(&db);

    let note = note_svc.create("Draft", "Version 1").unwrap();
    note_svc.update(&note.id, "Draft", "Version 2").unwrap();
    note_svc.update(&note.id, "Draft", "Version 3").unwrap();

    let history = hist_svc.list(&note.id).unwrap();
    assert_eq!(history.len(), 2); // two snapshots before two updates
    assert_eq!(history[0].content_snapshot, "Version 2");
    assert_eq!(history[1].content_snapshot, "Version 1");

    // Diff
    let diff = hist_svc.diff(&history[1].id, &history[0].id).unwrap();
    let has_change = diff.iter().any(|r| matches!(r, diff::Result::Right(_)));
    assert!(has_change);

    // Restore to v1
    let restored = hist_svc.restore(&history[1].id).unwrap();
    assert_eq!(restored.content, "Version 1");
}

#[test]
fn test_encrypted_export_import() {
    let db = setup_db();
    let note_svc = NoteService::new(&db);
    let export_svc = ExportService::new(&db);

    let note = note_svc.create("Secret", "Classified content").unwrap();
    let dir = tempfile::tempdir().unwrap();
    let zip_path = dir.path().join("export.zip");

    export_svc
        .export_notes(&[&note.id], &zip_path, "hunter2")
        .unwrap();

    let imported = export_svc.import_notes(&zip_path, "hunter2").unwrap();
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].title, "Secret");
    assert_eq!(imported[0].content, "Classified content");
}

#[test]
fn test_markdown_rendering() {
    let html = markdown::render_markdown("# Hello\n**bold**");
    assert!(html.contains("<h1>"));
    assert!(html.contains("<strong>bold</strong>"));

    let summary = markdown::plain_text_summary("# Hello\nWorld", 10);
    assert_eq!(summary, "HelloWorld");

    assert!(markdown::contains_math("$x^2$"));
    assert!(!markdown::contains_math("plain text"));

    let links = markdown::extract_wiki_links("See [[Note A]]");
    assert_eq!(links, vec!["Note A"]);
}

#[test]
fn test_tag_tree() {
    let db = setup_db();
    let tag_svc = TagService::new(&db);

    let root = tag_svc.create("work", None, None).unwrap();
    let child = tag_svc.create("projects", Some(&root.id), None).unwrap();
    tag_svc.create("internal", Some(&root.id), None).unwrap();

    // Children
    let children = tag_svc.get_children(&root.id).unwrap();
    assert_eq!(children.len(), 2);

    // Roots
    let roots = tag_svc.list_roots().unwrap();
    assert!(roots.iter().any(|t| t.id == root.id));

    // Update tag
    tag_svc
        .update(&child.id, "public-projects", None, Some("#ff0000"))
        .unwrap();
    let updated = tag_svc.get(&child.id).unwrap().unwrap();
    assert_eq!(updated.name, "public-projects");
    assert_eq!(updated.color.unwrap(), "#ff0000");

    // Delete tag cascades
    tag_svc.delete(&child.id).unwrap();
    assert!(tag_svc.get(&child.id).unwrap().is_none());
}

#[test]
fn test_search_filters() {
    let db = setup_db();
    let note_svc = NoteService::new(&db);
    let tag_svc = TagService::new(&db);
    let search_svc = SearchService::new(&db);

    let note = note_svc.create("API Design", "RESTful endpoints").unwrap();
    let tag = tag_svc.create("backend", None, None).unwrap();
    tag_svc.assign_to_note(&tag.id, &note.id).unwrap();

    // Search with tag filter
    let results = search_svc.search("tag:backend API").unwrap();
    assert!(!results.is_empty());
}
