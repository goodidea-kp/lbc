/*!
Example page: Table

AI Pair Programming Notes:
- Single-responsibility component focused on demonstrating the Table API.
- Keep imports minimal and explicit to reduce cognitive load.
- Keep examples deterministic and small; avoid hidden state outside this module.
*/

use lbc::prelude::{Block, Title, Subtitle, HeaderSize, Content, Table};
use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn TablePage() -> impl IntoView {
    view! {
        <Block>
            <Title size=HeaderSize::Is5>"Table"</Title>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Basic Table"</Subtitle>
                <Table>
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Position"</th>
                            <th>"Department"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Alice Johnson"</td>
                            <td>"Software Engineer"</td>
                            <td>"Engineering"</td>
                        </tr>
                        <tr>
                            <td>"Bob Smith"</td>
                            <td>"Product Manager"</td>
                            <td>"Product"</td>
                        </tr>
                        <tr>
                            <td>"Carol Davis"</td>
                            <td>"UX Designer"</td>
                            <td>"Design"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Bordered Table"</Subtitle>
                <Table bordered=true>
                    <thead>
                        <tr>
                            <th>"Product"</th>
                            <th>"Price"</th>
                            <th>"Stock"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Laptop"</td>
                            <td>"$999"</td>
                            <td>"15"</td>
                        </tr>
                        <tr>
                            <td>"Mouse"</td>
                            <td>"$29"</td>
                            <td>"50"</td>
                        </tr>
                        <tr>
                            <td>"Keyboard"</td>
                            <td>"$79"</td>
                            <td>"32"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Striped Table"</Subtitle>
                <Table striped=true>
                    <thead>
                        <tr>
                            <th>"Language"</th>
                            <th>"Year"</th>
                            <th>"Type"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Rust"</td>
                            <td>"2010"</td>
                            <td>"Systems"</td>
                        </tr>
                        <tr>
                            <td>"JavaScript"</td>
                            <td>"1995"</td>
                            <td>"Scripting"</td>
                        </tr>
                        <tr>
                            <td>"Python"</td>
                            <td>"1991"</td>
                            <td>"General Purpose"</td>
                        </tr>
                        <tr>
                            <td>"Go"</td>
                            <td>"2009"</td>
                            <td>"Systems"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Narrow Table"</Subtitle>
                <Table narrow=true bordered=true>
                    <thead>
                        <tr>
                            <th>"#"</th>
                            <th>"Item"</th>
                            <th>"Qty"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"1"</td>
                            <td>"Apple"</td>
                            <td>"5"</td>
                        </tr>
                        <tr>
                            <td>"2"</td>
                            <td>"Banana"</td>
                            <td>"12"</td>
                        </tr>
                        <tr>
                            <td>"3"</td>
                            <td>"Orange"</td>
                            <td>"8"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Hoverable Table"</Subtitle>
                <Table hoverable=true>
                    <thead>
                        <tr>
                            <th>"City"</th>
                            <th>"Country"</th>
                            <th>"Population"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Tokyo"</td>
                            <td>"Japan"</td>
                            <td>"37.4M"</td>
                        </tr>
                        <tr>
                            <td>"Delhi"</td>
                            <td>"India"</td>
                            <td>"31.1M"</td>
                        </tr>
                        <tr>
                            <td>"Shanghai"</td>
                            <td>"China"</td>
                            <td>"27.1M"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Fullwidth Table"</Subtitle>
                <Table fullwidth=true bordered=true striped=true>
                    <thead>
                        <tr>
                            <th>"Framework"</th>
                            <th>"Language"</th>
                            <th>"Paradigm"</th>
                            <th>"Release"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Leptos"</td>
                            <td>"Rust"</td>
                            <td>"Reactive"</td>
                            <td>"2022"</td>
                        </tr>
                        <tr>
                            <td>"React"</td>
                            <td>"JavaScript"</td>
                            <td>"Declarative"</td>
                            <td>"2013"</td>
                        </tr>
                        <tr>
                            <td>"Vue"</td>
                            <td>"JavaScript"</td>
                            <td>"Progressive"</td>
                            <td>"2014"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Scrollable Table"</Subtitle>
                <Content classes="mb-3">
                    <p>"The scrollable table is wrapped in a div.table-container for horizontal scrolling on smaller screens."</p>
                </Content>
                <Table scrollable=true fullwidth=true>
                    <thead>
                        <tr>
                            <th>"ID"</th>
                            <th>"First Name"</th>
                            <th>"Last Name"</th>
                            <th>"Email"</th>
                            <th>"Phone"</th>
                            <th>"Department"</th>
                            <th>"Location"</th>
                            <th>"Salary"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"001"</td>
                            <td>"John"</td>
                            <td>"Doe"</td>
                            <td>"john.doe@example.com"</td>
                            <td>"+1-555-1234"</td>
                            <td>"Engineering"</td>
                            <td>"San Francisco"</td>
                            <td>"$120,000"</td>
                        </tr>
                        <tr>
                            <td>"002"</td>
                            <td>"Jane"</td>
                            <td>"Smith"</td>
                            <td>"jane.smith@example.com"</td>
                            <td>"+1-555-5678"</td>
                            <td>"Marketing"</td>
                            <td>"New York"</td>
                            <td>"$95,000"</td>
                        </tr>
                        <tr>
                            <td>"003"</td>
                            <td>"Mike"</td>
                            <td>"Johnson"</td>
                            <td>"mike.j@example.com"</td>
                            <td>"+1-555-9012"</td>
                            <td>"Sales"</td>
                            <td>"Chicago"</td>
                            <td>"$85,000"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>

            <Block>
                <Subtitle size=HeaderSize::Is6>"Combined Modifiers"</Subtitle>
                <Table bordered=true striped=true narrow=true hoverable=true>
                    <thead>
                        <tr>
                            <th>"Feature"</th>
                            <th>"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Bordered"</td>
                            <td>"✓"</td>
                        </tr>
                        <tr>
                            <td>"Striped"</td>
                            <td>"✓"</td>
                        </tr>
                        <tr>
                            <td>"Narrow"</td>
                            <td>"✓"</td>
                        </tr>
                        <tr>
                            <td>"Hoverable"</td>
                            <td>"✓"</td>
                        </tr>
                    </tbody>
                </Table>
            </Block>
        </Block>
    }
}
