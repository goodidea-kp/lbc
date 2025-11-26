use lbc::prelude::{Block, Content, Control, Field, File, HeaderSize, Size, Subtitle, Title};
use leptos::prelude::{ IntoView, Set, component, view, signal};

#[component]
pub fn FormFilePage() -> impl IntoView {
    use std::sync::Arc;

    // In this catalog example we don't inspect real files; the File component
    // uses a platform-neutral placeholder type internally, so Vec<()> is fine.
    let (selected_files, set_selected_files) = signal(Vec::<()>::new());

    // Controlled component: update selected files on change.
    let on_update: Arc<dyn Fn(Vec<()>) + Send + Sync> = Arc::new(move |files: Vec<()>| {
        set_selected_files.set(files);
    });

    // Clone the callback for each usage inside the view macro to avoid move errors.
    let on_update_1 = on_update.clone();
    let on_update_2 = on_update.clone();

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: File"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic File"</Subtitle>
                <Field label="Upload a file" help="Select a file to upload">
                    <Control>
                        <File
                            name="upload"
                            _files=selected_files
                            _update=on_update_1
                            selector_label="Choose a file..."
                            has_name="No file selected"
                        />
                    </Control>
                </Field>
            </Content>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Boxed, Fullwidth, Right, Multiple"</Subtitle>
                <Field>
                    <Control>
                        <File
                            name="upload2"
                            _files=selected_files
                            _update=on_update_2
                            boxed=true
                            fullwidth=true
                            right=true
                            multiple=true
                            size=Size::Small
                            selector_label="Choose files..."
                            has_name="No files selected"
                        />
                    </Control>
                </Field>
            </Content>
        </Block>
    }
}
