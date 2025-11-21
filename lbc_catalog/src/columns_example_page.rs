/*!
Example page: Columns

AI Pair Programming Notes:
- Shows responsive Columns + Column sizing with clear, labeled content.
*/

use lbc::prelude::{Block, Title, HeaderSize, Notification, Column, ColumnSize, Columns};
use leptos::prelude::{ClassAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn ColumnsPage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Columns"</Title>
            <Columns centered=true multiline=true>
                <Column size=ColumnSize::OneThird>
                    <Notification classes="is-primary">"One Third"</Notification>
                </Column>
                <Column size=ColumnSize::OneThird>
                    <Notification classes="is-warning">"One Third"</Notification>
                </Column>
                <Column size=ColumnSize::OneThird>
                    <Notification classes="is-info">"One Third"</Notification>
                </Column>
            </Columns>
        </Block>
    }
}
