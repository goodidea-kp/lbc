use leptos::prelude::{component, view, IntoView, ClassAttribute, ElementChild, Get, Set, create_signal};
use lbc::prelude::{Block, Content, Control, Field, HeaderSize, Size, Subtitle, TextArea, Title};
use std::sync::Arc;

/// Example page showcasing the TextArea form component.
#[component]
pub fn FormTextAreaPage() -> impl IntoView {
    let (notes, set_notes) = create_signal(String::new());
    let (bio, set_bio) = create_signal(String::from("Once upon a time..."));
    let (ai_text, set_ai_text) = create_signal(String::new());

    let update_notes = Arc::new(move |v: String| set_notes.set(v));
    let update_bio = Arc::new(move |v: String| set_bio.set(v));
    let update_ai = Arc::new(move |v: String| set_ai_text.set(v));

    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Form: TextArea"</Title>

            <Content>
                <Subtitle size=HeaderSize::Is6>"Basic TextArea"</Subtitle>
                <Field label="Notes" help="Enter a few lines">
                    <Control>
                        <TextArea
                            name="notes"
                            value=notes
                            rows=4
                            placeholder="Type some notes…"
                            update=update_notes.clone()
                        />
                    </Control>
                </Field>
                <p class="help">"Notes: " {move || notes.get()}</p>

                <Subtitle size=HeaderSize::Is6>"Sizes & Flags"</Subtitle>
                <Field>
                    <Control>
                        <TextArea
                            name="bio"
                            value=bio
                            rows=6
                            size=Size::Small
                            placeholder="Tell us your story…"
                            loading=false
                            fixed_size=true
                            update=update_bio.clone()
                        />
                    </Control>
                </Field>

                <Subtitle size=HeaderSize::Is6>"With GenAI ribbon"</Subtitle>
                <Field>
                    <Control>
                        <TextArea
                            name="ai"
                            value=ai_text
                            rows=3
                            is_genai=true
                            placeholder="Generated text…"
                            update=update_ai.clone()
                        />
                    </Control>
                </Field>
            </Content>
        </Block>
    }
}
