// NOTE: This code accompanies the "Attributes in Leptos v0.7" article.
//       It demonstrates typed children, dynamic classes, Show fallback,
//       array/slice dynamic class syntax, an AttributeInterceptor example, and more.

use leptos::attribute_interceptor::AttributeInterceptor; // For the "Spread Props" example
use leptos::prelude::*;
use leptos_typed_fallback_show::TypedFallbackShow;

//////////////////////////
// 1. Examples Module  //
//////////////////////////

pub mod examples {
    use super::*;

    //----------------------------------------
    // A) "Does Not Pass Through" Example
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn ComponentDoesNotPass(children: Children) -> impl IntoView {
        // Because this uses `AnyView`, attribute passthrough won't apply.
        view! {
            {children()}
        }
    }

    #[component]
    #[allow(non_snake_case)]
    pub fn RootDoesNotPass() -> impl IntoView {
        view! {
            /* Attempt to pass <code>class="red-box"</code>
               via <code>attr:class</code> or <code>{..} class</code> */
            <ComponentDoesNotPass attr:class="red-box">
                <div style:padding="0.5rem">
                    "Should have red background if class passed... but it won't!"
                </div>
            </ComponentDoesNotPass>
        }
    }

    //----------------------------------------
    // B) "TypedChildren" - Passes Through
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn ComponentPasses(children: TypedChildren<impl IntoView + 'static>) -> impl IntoView {
        // Because this uses <code>TypedChildren</code>,
        // the <code>attr:class</code> *does* pass.
        view! {
            {children.into_inner()()}
        }
    }

    #[component]
    #[allow(non_snake_case)]
    pub fn RootPasses() -> impl IntoView {
        view! {
            <ComponentPasses attr:class="foo">
                <div style:padding="0.5rem">
                    "We should get a green border if "
                    <code>"foo"</code>
                    " is applied!"
                </div>
            </ComponentPasses>
        }
    }

    //----------------------------------------
    // C1) Dynamic Class: Parent-Defined
    //----------------------------------------
    // The parent sets a dynamic class, merging with
    // the child's static classes "bar baz" => "bar baz foo"

    #[component]
    #[allow(non_snake_case)]
    pub fn RootParentDynamicClass() -> impl IntoView {
        view! {
            <ComponentPasses class:foo=true>
                <div class="bar baz" style:padding="0.5rem">
                    "(Parent) Dynamic class: 'foo'; static classes: 'bar baz' -> 'bar baz foo'"
                </div>
            </ComponentPasses>
        }
    }

    //----------------------------------------
    // C2) Dynamic Class: Child-Defined
    //----------------------------------------
    // The parent sets static classes, the child
    // tries to add "foo", but gets overridden => "bar baz"

    #[component]
    #[allow(non_snake_case)]
    pub fn RootChildDynamicClass() -> impl IntoView {
        view! {
            <ComponentPasses attr:class="bar baz">
                <div class:foo=true style:padding="0.5rem">
                    "(Parent) Static classes: 'bar baz'; (Child) dynamic class: 'foo' -> 'bar baz'"
                </div>
            </ComponentPasses>
        }
    }

    //----------------------------------------
    // D) Show Example with Fallback
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn RootShow() -> impl IntoView {
        let (show, set_show) = signal(true);

        // The fallback uses `AnyView` internally => attributes won't pass
        view! {
            <div style:margin-bottom="1rem">
                <button on:click=move |_| set_show.update(|v| *v = !*v)>
                    "Toggle Show"
                </button>
                <Show
                    when=move || show.get()
                    fallback=|| view! {
                        <span style:padding="0.5rem">
                            "Fallback (no "
                            <code>"foo"</code>
                            " class applied!)"
                        </span>
                    }
                    attr:class="foo"
                >
                    <div style:padding="0.5rem">
                        "Main Content (has "
                        <code>"foo"</code>
                        " class!)"
                    </div>
                </Show>
            </div>
        }
    }

    //----------------------------------------
    // E) TypedFallbackShow Example
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn RootTypedFallbackShow() -> impl IntoView {
        let (show, set_show) = signal(true);

        view! {
            <div style:margin-bottom="1rem">
                <button on:click=move |_| set_show.update(|v| *v = !*v)>
                    "Toggle Show"
                </button>
                <TypedFallbackShow
                    when=move || show.get()
                    fallback=|| view! {
                        <span style:padding="0.5rem">
                            "Fallback (will have "
                            <code>"foo"</code>
                            " class!)"
                        </span>
                    }
                    attr:class="foo"
                >
                    <div style:padding="0.5rem">
                        "Main Content (has "
                        <code>"foo"</code>
                        " class!)"
                    </div>
                </TypedFallbackShow>
            </div>
        }
    }

    //----------------------------------------
    // F1) Many Dynamic Classes Example
    // (Showcases 26-class limit)
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn RootManyClasses() -> impl IntoView {
        view! {
            <ComponentPasses
                // Outer component: 26 dynamic classes
                class:c1=true
                class:c2=true
                class:c3=true
                class:c4=true
                class:c5=true
                class:c6=true
                class:c7=true
                class:c8=true
                class:c9=true
                class:c10=true
                class:c11=true
                class:c12=true
                class:c13=true
                class:c14=true
                class:c15=true
                class:c16=true
                class:c17=true
                class:c18=true
                class:c19=true
                class:c20=true
                class:c21=true
                class:c22=true
                class:c23=true
                class:c24=true
                class:c25=true
                class:c26=true
                // class:c27=true // Uncomment to see the trait bound error
            >
                // Inner component: just 1 more class (c27),
                // letting us surpass the 26-class limit if needed.
                <ComponentPasses class:c27=true>
                    <div>
                        "This div has classes c1...c27 → more than 26 dynamically applied classes!"
                    </div>
                </ComponentPasses>
            </ComponentPasses>
        }
    }

    //----------------------------------------
    // F2) Many Dynamic Attributes Example
    // (Showcases 26-attribute limit)
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn RootManyAttributes() -> impl IntoView {
        view! {
            // Outer component: 26 dynamic attributes (mixed style + event)
            <ComponentPasses
                style:width="50px"
                style:height="50px"
                style:color="red"
                style:background_color="blue"
                style:border="1px solid black"
                style:padding_left="5px"
                style:padding_right="5px"
                style:margin_left="5px"
                style:margin_right="5px"
                style:font_weight="bold"
                on:click=|_| println!("Clicked 1!")
                on:dblclick=|_| println!("Double-clicked 2!")
                on:mouseenter=|_| println!("Mouse entered 3!")
                on:mouseleave=|_| println!("Mouse left 4!")
                on:focus=|_| println!("Focused 5!")
                on:blur=|_| println!("Blurred 6!")
                on:keydown=|_| println!("Key down 7!")
                on:keyup=|_| println!("Key up 8!")
                on:keypress=|_| println!("Key press 9!")
                on:wheel=|_| println!("Wheel 10!")
                on:copy=|_| println!("Copy 11!")
                on:paste=|_| println!("Paste 12!")
                on:cut=|_| println!("Cut 13!")
                on:contextmenu=|_| println!("Context menu 14!")
                style:font_size="20px"
                style:font_family="sans-serif"
                // style:overflow="hidden" // Uncomment to see the trait bound error (27th)
            >
                // Inner component: one more dynamic attribute, effectively the 27th if uncommented above
                <ComponentPasses style:overflow="auto">
                    <div>
                        "Testing style + event attributes. This div now has effectively 27 dynamic attributes
                        if you uncomment the 27th on the outer level!"
                    </div>
                </ComponentPasses>
            </ComponentPasses>
        }
    }

    //----------------------------------------
    // G) Array/Slice Dynamic Classes on a Component vs. DOM
    //----------------------------------------

    // 1) Illustrate the compile-time error
    #[component]
    #[allow(non_snake_case)]
    pub fn RootArraySliceCompileError() -> impl IntoView {
        // Attempting the below code would trigger a compile error:
        //
        // view! {
        //     <ComponentPasses {..} class=(["foo", "qux", "quux"], true)>
        //         <div class="bar baz">
        //             "Won't compile because the macro doesn't support array/slice dynamic classes on components."
        //         </div>
        //     </ComponentPasses>
        // }
        //
        // Instead, we'll just display a message:
        view! {
            <div style:padding="0.5rem">
                <p style:color="red">
                    "Array/Slice syntax for classes on a "
                    <code>"Component"</code>
                    " doesn't compile. Check the commented-out code in this component for details."
                </p>
            </div>
        }
    }

    // 2) Show that array/slice classes DO work for DOM elements
    #[component]
    #[allow(non_snake_case)]
    pub fn RootArraySliceWorks() -> impl IntoView {
        view! {
            <ComponentPasses {..} style:padding="0.5rem" class:bar=true class:baz=true>
                <div class=(["foo", "qux", "quux"], true)>
                    "Here, we used the slice/array syntax for a DOM element,
                     resulting in 'foo qux quux' plus the existing 'bar baz' from the parent."
                </div>
            </ComponentPasses>
        }
    }

    //----------------------------------------
    // H) AttributeInterceptor Example
    //----------------------------------------
    // Demonstrates how to "spread props" onto a child element

    // First, create a reusable table wrapper component
    #[component]
    #[allow(non_snake_case)]
    fn DataTable() -> impl IntoView {
        view! {
            <AttributeInterceptor let:attrs>
                <div style:padding="1rem" class="wrapper">
                    <table {..attrs} class="data-table">
                        <thead>
                            <tr>
                                <th>"Product"</th>
                                <th>"Price"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td>"Widget"</td><td>"$10.00"</td></tr>
                            <tr><td>"Gadget"</td><td>"$15.00"</td></tr>
                        </tbody>
                    </table>
                </div>
            </AttributeInterceptor>
        }
    }

    // Then demonstrate its usage with attributes that will be intercepted
    #[component]
    #[allow(non_snake_case)]
    pub fn RootAttributeInterceptor() -> impl IntoView {
        view! {
            <div>
                <DataTable
                    style:border="2px solid blue"
                    attr:data-test-id="product-table"
                    attr:role="grid"
                />
                <p>
                    "Inspect the table element - it received the border style,
                    data-test-id, and role attributes!"
                </p>
            </div>
        }
    }

    //----------------------------------------
    // I) Deep Attribute Nesting Stress Test
    //----------------------------------------

    #[component]
    #[allow(non_snake_case)]
    pub fn RootDeepAttributeNesting() -> impl IntoView {
        #[component]
        #[allow(non_snake_case)]
        fn NestedComponent(
            #[prop(into, optional, default = 0.into())] 
            level: MaybeProp<i32>,
            #[prop(into)] 
            max: i32
        ) -> impl IntoView {
            let current_level = level.get().unwrap_or(0);
    
            let background_color = match current_level % 6 {
                0 => "#fafaff",
                1 => "#fffaf0",
                2 => "#f5fff5",
                3 => "#f9f9ff",
                4 => "#fdf5f5",
                _ => "#f5ffff",
            };
    
            let padding = 4 + current_level * 1;
            let margin = 2 + current_level * 1;
            let style = format!("padding: {}px 0; margin: {}px 0; background-color: {}; border-radius: 4px",
                padding, margin, background_color);
    
            if current_level >= max {
                view! {
                    <div
                        style=format!("{}; border: 1px solid {}", style, background_color)
                    >
                        <strong>"Reached max nesting: "</strong>
                        {current_level}
                    </div>
                }.into_any()
            } else {
                view! {
                    <ComponentDoesNotPass attr:data-level-outer=current_level.to_string() attr:style=format!("{}; border: 1px dashed {}", style, background_color)>
                        <div>
                            <strong>"Level: "</strong>{current_level}
                            <NestedComponent attr:data-level-inner=current_level.to_string() level={current_level + 1} max/>
                        </div>
                    </ComponentDoesNotPass>
                }.into_any()
            }
        }
    
        view! {
            <div style="font-family: Arial, sans-serif; padding: 10px;">
                <h3>"Deep Attribute Nesting Stress Test"</h3>
                <p>
                    "This test recursively nests "
                    <code>"ComponentPasses"</code>
                    " up to many levels, each adding a unique "
                    <code>"data-level"</code>
                    " attribute."
                </p>
                <NestedComponent level=0 max=30/>
            </div>
        }
    }
}

//////////////////////////
// 2. Playground Root  //
//////////////////////////

#[component]
fn PlaygroundRoot() -> impl IntoView {
    view! {
        <div style:padding="1rem" style:font-family="sans-serif">
            <style>{r#"
code {
  background-color: #f2f2f2;
  font-family: monospace;
  padding: 0.125rem 0.25rem;
  border-radius: 4px;
}

.red-box {
  background-color: #ffdddd;
  border: 1px solid red;
  margin: 0.5rem 0;
}

.foo {
  background-color: #ddffdd;
  border: 1px solid green;
  border-radius: 4px;
  margin: 0.5rem 0;
}

.bar {
  background-color: #ddddff;
}

.baz {
  border: 2px dashed blue;
}

.outer {
  padding: 0.5rem;
  border: 1px solid purple;
}

.inner {
  background-color: #fffdd0;
  margin-top: 0.5rem;
}

button {
  padding: 0.25rem 0.5rem;
  margin-top: 0.25rem;
  cursor: pointer;
}

.c1, .c2, .c3, .c4, .c5, .c6, .c7, .c8, .c9, .c10,
.c11, .c12, .c13, .c14, .c15, .c16, .c17, .c18, .c19, .c20,
.c21, .c22, .c23, .c24, .c25, .c26 {
  outline: 1px dotted #333;
}

.data-table {
    border-collapse: collapse;
    width: 100%;
}
.data-table th, .data-table td {
    border: 1px solid #ddd;
    padding: 8px;
}
.data-table th {
    background-color: #f8f8f8;
}
"#}</style>

            <h1>"Leptos Attribute System Playground"</h1>
            <p>
                "Below are multiple examples validating code snippets from the article.
                 You can inspect each in dev tools or watch the visual changes on the page."
            </p>

            <hr/>
            <h2>"1) Does Not Pass Through"</h2>
            <examples::RootDoesNotPass/>
            <p>
                "Check the rendered DOM: the parent tries to set "
                <code>"class=\"red-box\""</code>
                " on the child, but it doesn't appear because "
                <code>"Children"</code>
                " uses "
                <code>"AnyView"</code>
                " internally."
            </p>

            <hr/>
            <h2>"2) Does Pass Through"</h2>
            <examples::RootPasses/>
            <p>
                "Here, using "
                <code>"TypedChildren"</code>
                " successfully applies the "
                <code>"class=\"foo\""</code>
                " to the child "
                <code>"<div/>"</code>
                ", giving it a green border."
            </p>

            <hr/>
            <h2>"3) Parent-Defined Dynamic Class"</h2>
            <examples::RootParentDynamicClass/>
            <p>
                "The parent sets a dynamic class "
                <code>"foo=true"</code>
                ". The child has static classes "
                <code>"bar baz"</code>
                ". Resulting in "
                <code>"<div class=\"bar baz foo\"/>"</code>
                "."
            </p>

            <hr/>
            <h2>"4) Child-Defined Dynamic Class"</h2>
            <examples::RootChildDynamicClass/>
            <p>
                "The parent statically defines "
                <code>"bar baz"</code>
                ", while the child tries to add "
                <code>"foo"</code>
                " dynamically. Because the parent's static classes override the child’s,
                the result remains "
                <code>"bar baz"</code>
                "."
            </p>

            <hr/>
            <h2>"5) Show Example (Fallback Loses Attr)"</h2>
            <examples::RootShow/>
            <p>
                "Toggle the signal. The fallback branch won't get "
                <code>"class=\"foo\""</code>
                " because "
                <code>"AnyView"</code>
                " discards it internally."
            </p>

            <hr/>
            <h2>"6) TypedFallbackShow Example"</h2>
            <examples::RootTypedFallbackShow/>
            <p>
                "Toggle the signal. Both the main content and the fallback now receive "
                <code>"class=\"foo\""</code>
                "."
            </p>

            <hr/>
            <h2>"7) Many Dynamic Classes (26 Limit)"</h2>
            <examples::RootManyClasses/>
            <p>
                "This example adds 26 dynamic classes using "
                <code>"class:"</code>
                " syntax. Uncomment the 27th to see a compilation error."
            </p>

            <hr/>
            <h2>"7.2) Many Dynamic Attributes (26 Limit)"</h2>
            <examples::RootManyAttributes/>
            <p>
                "Similarly, this example adds 26 total dynamic attributes (mixed style & event).
                Uncomment the 27th to exceed the limit."
            </p>

            <hr/>
            <h2>"8) Array/Slice Dynamic Classes on a Component vs. DOM"</h2>
            <examples::RootArraySliceCompileError/>
            <p>
                "The array/slice syntax "
                <code>"class=([\"foo\", \"qux\", \"quux\"], true)"</code>
                " fails on "
                <code>"ComponentPasses"</code>
                " or any component macro, causing a compile error."
            </p>
            <examples::RootArraySliceWorks/>
            <p>
                "However, the exact same syntax works for DOM elements. Inspect the child "
                <code>"<div>"</code>
                " and you should see classes: "
                <code>"foo qux quux bar baz"</code>
                "."
            </p>

            <hr/>
            <h2>"9) Using AttributeInterceptor (Spread Props)"</h2>
            <examples::RootAttributeInterceptor/>
            <p>
                "Here, "
                <code>"AttributeInterceptor"</code>
                " captures all attributes passed to this component and spreads them onto
                the inner "
                <code>"<table>"</code>
                ". This lets you wrap the table in a "
                <code>"<div>"</code>
                " while still letting users apply their own attributes."
            </p>

            <hr/>

            <h2>"10) Deep Attribute Nesting Stress Test"</h2>
            <examples::RootDeepAttributeNesting/>
            <p>
                "This test nests <ComponentPasses> 100 times, each adding a unique attribute.
                If the compiler fails with the previous issue, it will not compile or will take excessively long."
            </p>
        </div>
    }
}

//////////////////////////
// 3. Main Entry Point //
//////////////////////////

pub fn main() {
    // For a simple client-side run, just mount to body:
    mount_to_body(|| view! { <PlaygroundRoot/> });
}
