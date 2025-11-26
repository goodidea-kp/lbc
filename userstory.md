
# LBC Test Attributes Implementation Plan                                                                                                                                                                                                                       


## User Story 
# S.3.1: Add Optional Test Attributes to All LBC Components

As a developer using LBC components                                                                                                                                                                                                                              
I want to add test attributes (like data-testid) to any component                                                                                                                                                                                                
So that I can easily identify and test components in my test suites

Acceptance Criteria

• AC-1: All components support an optional test_id prop that renders as data-testid attribute                                                                                                                                                                   
• AC-2: Components with dynamic tags apply the test attribute to the rendered element                                                                                                                                                                           
• AC-3: WASM tests verify test attribute presence/absence for each component


Implementation Steps

## Step 1: Update Cargo.toml

Add wasm-bindgen-test to dev dependencies:


[dev-dependencies]                                                                                                                                                                                                                                               
leptos = { version = "0.8.12", features = ["ssr"] }                                                                                                                                                                                                              
wasm-bindgen-test = "0.3"

[target.'cfg(target_arch = "wasm32")'.dev-dependencies]                                                                                                                                                                                                          
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlElement"] }


## Step 2: Component Updates

Add these props to EVERY component:


/// Optional test identifier (renders as data-testid attribute)                                                                                                                                                                                                  
#[prop(optional, into)] test_id: Option<String>,


Add the attribute to the root element:


data-testid=test_id


## Step 3: Test Template

Add to each component file:


#[cfg(all(test, target_arch = "wasm32"))]                                                                                                                                                                                                                        
mod wasm_tests {                                                                                                                                                                                                                                                 
use super::*;                                                                                                                                                                                                                                                
use leptos::prelude::*;                                                                                                                                                                                                                                      
use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);                                                                                                                                                                                                                
                                                                                                                                                                                                                                                                 
    #[wasm_bindgen_test]                                                                                                                                                                                                                                         
    fn component_renders_test_id() {                                                                                                                                                                                                                             
        let html = view! {                                                                                                                                                                                                                                       
            <ComponentName test_id="test-component">"Content"</ComponentName>                                                                                                                                                                                    
        }.to_html();                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                 
        assert!(                                                                                                                                                                                                                                                 
            html.contains(r#"data-testid="test-component""#),                                                                                                                                                                                                    
            "expected data-testid attribute; got: {}",                                                                                                                                                                                                           
            html                                                                                                                                                                                                                                                 
        );                                                                                                                                                                                                                                                       
    }                                                                                                                                                                                                                                                            
                                                                                                                                                                                                                                                                 
    #[wasm_bindgen_test]                                                                                                                                                                                                                                         
    fn component_no_test_id_when_not_provided() {                                                                                                                                                                                                                
        let html = view! {                                                                                                                                                                                                                                       
            <ComponentName>"Content"</ComponentName>                                                                                                                                                                                                             
        }.to_html();                                                                                                                                                                                                                                             
                                                                                                                                                                                                                                                                 
        assert!(                                                                                                                                                                                                                                                 
            !html.contains("data-testid"),                                                                                                                                                                                                                       
            "expected no data-testid attribute; got: {}",                                                                                                                                                                                                        
            html                                                                                                                                                                                                                                                 
        );                                                                                                                                                                                                                                                       
    }                                                                                                                                                                                                                                                            
}



Components Checklist

Elements (12 files)

• [ ] src/elements/block.rs - Block                                                                                                                                                                                                                             
• [ ] src/elements/box.rs - Box                                                                                                                                                                                                                                 
• [ ] src/elements/button.rs - Button                                                                                                                                                                                                                           
• [ ] src/elements/content.rs - Content (dynamic tag)                                                                                                                                                                                                           
• [ ] src/elements/delete.rs - Delete (dynamic tag)                                                                                                                                                                                                             
• [ ] src/elements/icon.rs - Icon                                                                                                                                                                                                                               
• [ ] src/elements/notification.rs - Notification                                                                                                                                                                                                               
• [ ] src/elements/progress.rs - Progress                                                                                                                                                                                                                       
• [ ] src/elements/table.rs - Table                                                                                                                                                                                                                             
• [ ] src/elements/tag.rs - Tag                                                                                                                                                                                                                                 
• [ ] src/elements/title.rs - Title (dynamic tag), Subtitle (dynamic tag)

Form (9 files)

• [ ] src/form/checkbox.rs - Checkbox                                                                                                                                                                                                                           
• [ ] src/form/control.rs - Control (dynamic tag)                                                                                                                                                                                                               
• [ ] src/form/field.rs - Field                                                                                                                                                                                                                                 
• [ ] src/form/file.rs - File                                                                                                                                                                                                                                   
• [ ] src/form/input.rs - Input                                                                                                                                                                                                                                 
• [ ] src/form/radio.rs - Radio                                                                                                                                                                                                                                 
• [ ] src/form/select.rs - Select, MultiSelect                                                                                                                                                                                                                  
• [ ] src/form/textarea.rs - TextArea

Layout (8 files)

• [ ] src/layout/columns.rs - Columns, Column                                                                                                                                                                                                                   
• [ ] src/layout/container.rs - Container                                                                                                                                                                                                                       
• [ ] src/layout/footer.rs - Footer                                                                                                                                                                                                                             
• [ ] src/layout/hero.rs - Hero                                                                                                                                                                                                                                 
• [ ] src/layout/level.rs - Level (dynamic tag), LevelLeft, LevelRight, LevelItem (dynamic tag)                                                                                                                                                                 
• [ ] src/layout/media.rs - Media (dynamic tag), MediaLeft, MediaRight, MediaContent (all dynamic tags)                                                                                                                                                         
• [ ] src/layout/section.rs - Section                                                                                                                                                                                                                           
• [ ] src/layout/tile.rs - Tile (dynamic tag)

Components (13 files) - Need to add to chat

• [ ] src/components/accordion.rs - Accordions, AccordionItem                                                                                                                                                                                                   
• [ ] src/components/breadcrumb.rs - Breadcrumb                                                                                                                                                                                                                 
• [ ] src/components/calendar.rs - Calendar                                                                                                                                                                                                                     
• [ ] src/components/card.rs - Card, CardHeader, CardImage, CardContent, CardFooter                                                                                                                                                                             
• [ ] src/components/dropdown.rs - Dropdown                                                                                                                                                                                                                     
• [ ] src/components/menu.rs - Menu, MenuList, MenuLabel                                                                                                                                                                                                        
• [ ] src/components/message.rs - Message, MessageHeader, MessageBody                                                                                                                                                                                           
• [ ] src/components/modal.rs - Modal, ModalCard                                                                                                                                                                                                                
• [ ] src/components/navbar.rs - Navbar, NavbarItem, NavbarDropdown, NavbarDivider                                                                                                                                                                              
• [ ] src/components/pagination.rs - Pagination, PaginationItem, PaginationEllipsis                                                                                                                                                                             
• [ ] src/components/panel.rs - Panel, PanelBlock (dynamic tag), PanelTabs                                                                                                                                                                                      
• [ ] src/components/tabs.rs - Tabs


Special Handling for Dynamic Tag Components

For components that render different HTML elements based on a tag prop:


match tag_name.as_str() {                                                                                                                                                                                                                                        
"article" => view! { <article class=class_attr data-testid=test_id>{children()}</article> }.into_any(),                                                                                                                                                      
"section" => view! { <section class=class_attr data-testid=test_id>{children()}</section> }.into_any(),                                                                                                                                                      
"nav" => view! { <nav class=class_attr data-testid=test_id>{children()}</nav> }.into_any(),                                                                                                                                                                  
_ => view! { <div class=class_attr data-testid=test_id>{children()}</div> }.into_any(),                                                                                                                                                                      
}



Testing Commands


# Run all tests including WASM tests
wasm-pack test --headless --chrome

# Run specific component tests
wasm-pack test --headless --chrome -- --test elements::button::wasm_tests



Example Implementation

Simple Component (Button)


#[component]                                                                                                                                                                                                                                                     
pub fn Button(                                                                                                                                                                                                                                                   
// ... existing props ...                                                                                                                                                                                                                                    
/// Optional test identifier (renders as data-testid attribute)                                                                                                                                                                                              
#[prop(optional, into)] test_id: Option<String>,                                                                                                                                                                                                             
children: Children,                                                                                                                                                                                                                                          
) -> impl IntoView {                                                                                                                                                                                                                                             
// ... existing implementation ...

    view! {                                                                                                                                                                                                                                                      
        <button                                                                                                                                                                                                                                                  
            class=class                                                                                                                                                                                                                                          
            disabled=move || disabled.get()                                                                                                                                                                                                                      
            data-testid=test_id                                                                                                                                                                                                                                  
            on:click=move |event| {                                                                                                                                                                                                                              
                if let Some(cb) = on_click_callback.as_ref() {                                                                                                                                                                                                   
                    (cb)(event);                                                                                                                                                                                                                                 
                }                                                                                                                                                                                                                                                
            }                                                                                                                                                                                                                                                    
        >                                                                                                                                                                                                                                                        
            {children()}                                                                                                                                                                                                                                         
        </button>                                                                                                                                                                                                                                                
    }                                                                                                                                                                                                                                                            
}


Dynamic Tag Component (Content)


#[component]                                                                                                                                                                                                                                                     
pub fn Content(                                                                                                                                                                                                                                                  
// ... existing props ...                                                                                                                                                                                                                                    
/// Optional test identifier (renders as data-testid attribute)                                                                                                                                                                                              
#[prop(optional, into)] test_id: Option<String>,                                                                                                                                                                                                             
children: Children,                                                                                                                                                                                                                                          
) -> AnyView {                                                                                                                                                                                                                                                   
// ... existing implementation ...

    match tag_name.as_str() {                                                                                                                                                                                                                                    
        "article" => view! { <article class=class_attr.clone() data-testid=test_id.clone()>{children()}</article> }.into_any(),                                                                                                                                  
        "section" => view! { <section class=class_attr.clone() data-testid=test_id.clone()>{children()}</section> }.into_any(),                                                                                                                                  
        "nav" => view! { <nav class=class_attr.clone() data-testid=test_id.clone()>{children()}</nav> }.into_any(),                                                                                                                                              
        "p" => view! { <p class=class_attr.clone() data-testid=test_id.clone()>{children()}</p> }.into_any(),                                                                                                                                                    
        "span" => view! { <span class=class_attr.clone() data-testid=test_id.clone()>{children()}</span> }.into_any(),                                                                                                                                           
        _ => view! { <div class=class_attr.clone() data-testid=test_id>{children()}</div> }.into_any(),                                                                                                                                                          
    }                                                                                                                                                                                                                                                            
}



Total Components to Update: 62

• Elements: 12 components                                                                                                                                                                                                                                       
• Form: 9 components                                                                                                                                                                                                                                            
• Layout: 15 components                                                                                                                                                                                                                                         
• Components: 26 components


Success Criteria

• [ ] All 62 components accept test_id prop                                                                                                                                                                                                                     
• [ ] All components render data-testid attribute when provided                                                                                                                                                                                                 
• [ ] All components have WASM tests verifying the functionality                                                                                                                                                                                                
• [ ] No breaking changes to existing API                                                                                                                                                                                                                       
• [ ] Documentation updated with test_id prop  