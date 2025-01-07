# Attributes in Leptos v0.7

[![Leptos](https://img.shields.io/badge/Leptos-0.7.3-green?style=flat-square&logo=rust)](https://crates.io/crates/leptos/0.7.3) [![CodeSandbox](https://img.shields.io/badge/Open%20in-CodeSandbox-blue?style=flat-square&logo=codesandbox)](https://codesandbox.io/p/devbox/priceless-ace-mqfmml?embed=1&hidenavigation=1)
[![HackMD](https://img.shields.io/badge/Read%20on-HackMD-black?style=flat-square&logo=hackmd)](https://hackmd.io/@mJnsnK9eTqSUJ_WudSrPEQ/HkMv4D5Lyx)

With the introduction of Leptos v0.7, the framework’s internal design has achieved significant efficiency gains and elegance. However, these improvements have introduced a few challenges related to attribute handling that took some time to navigate. This document aims to outline these challenges and provide guidance to help others avoid similar frustrations. To date, the new attribute system in Leptos has successfully supported existing design patterns, though there are a few areas where further enhancements would be beneficial.

# Table of Contents

- [Attributes in Leptos v0.7](#Attributes-in-Leptos-v07)
  - [Handling Attribute Passthrough](#Handling-Attribute-Passthrough)
    - [Attribute Passthrough Illustrated](#Attribute-Passthrough-Illustrated)
    - [Handling Attributes with Show](#Handling-Attributes-with-Show)
  - [Static & Dynamic Class Attribute Precedence](#Static-amp-Dynamic-Class-Attribute-Precedence)
    - [Static Classes](#Static-Classes)
    - [Dynamic Classes](#Dynamic-Classes)
      - [1. Parent-Defined Dynamic Class](#1-Parent-Defined-Dynamic-Class)
      - [2. Child-Defined Dynamic Class](#2-Child-Defined-Dynamic-Class)
      - [3. When Dynamic Classes Can't Be Parsed by the View Macro](#3-When-Dynamic-Classes-Cant-Be-Parsed-by-the-View-Macro)
      - [4. The 26-Attribute Limit & Double-Nested Components](#4-The-26-Attribute-Limit-&-Double-Nested-Components)
      - [5. Dynamic Class Array Syntax](#5-Dynamic-Class-Array-Syntax)
  - [Using AttributeInterceptor (a.k.a. "Spread Props")](#Using-AttributeInterceptor-aka-%E2%80%9CSpread-Props%E2%80%9D)
  - [Custom Class Merging Logic](#Custom-Class-Merging-Logic)
  - [Conclusion & Recommendations](#Conclusion-amp-Recommendations)

## Handling Attribute Passthrough

Leptos 0.7 introduced a powerful concept where attributes “spread” onto a component flow down to **all** elements within that component’s returned view. As stated in the [0.7.0 release notes](https://github.com/leptos-rs/leptos/releases/tag/v0.7.0):

> “Attributes that are spread onto a component will be applied to _all_ elements returned as part of the component’s view. To apply attributes to a subset of the component, pass them via a component prop.”

We refer to this cascading behavior as “attribute passthrough.” In most cases, this mechanism works exactly as expected. However, in Leptos 0.7.3, there is a key limitation: attributes do not pass through to components that use AnyView internally. This constraint is not intentional but rather stems from an implementation challenge. Consequently, constructs like `Children`, `ChildrenFragment`, and `ChildrenFn`—all of which rely on `AnyView`—will not support attribute passthrough. If you’re interested in the technical details, check out [Leptos PR #2905](https://github.com/leptos-rs/leptos/pull/2905) or the [relevant Discord discussion](https://discord.com/channels/1031524867910148188/1321407611366539275).

The following sections highlight these nuances, explain where attribute passthrough falls short, and show how to work around its limitations. By understanding these edge cases, you can make the most of the attribute system’s flexibility and avoid unexpected pitfalls in more complex designs.

### Attribute Passthrough Illustrated

#### **Issue: Attributes Do Not Pass Through**

**Does Not Work as Intended ❌**

```rust
#[component]
#[allow(non_snake_case)]
pub fn Component(children: Children) -> impl IntoView {
    view! {
        {children()}
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Root() -> impl IntoView {
    view! {
        <Component attr:class="foo"> // or <Component {..} class="foo">
            <div/>
        </Component>
    } // Renders as <div/>
}
```

_In this example, the `class="foo"` attribute intended for `<Component>` does not pass through to the `<div/>` child._

**Why is this?**

`AddAnyAttr` is implemented for `AnyView`, but it simply returns `AnyView` without applying any effect. You can view the relevant implementation [here](https://github.com/leptos-rs/leptos/blob/165911b2e676bd4302cdfe890610012c070d6d83/tachys/src/view/any_view.rs#L322-L334). Essentially, calling `add_any_attr(<attr>)` on `AnyView` currently has no impact. If you’re interested in helping fix this, check out [Leptos PR #2905](https://github.com/leptos-rs/leptos/pull/2905).

##### Children Types That Do **Not** Support Passthrough

- `Children`
- `ChildrenFragment`
- `ChildrenFn`
- `ChildrenFragmentFn`
- `ChildrenFnMut`
- `ChildrenFragmentMut`
- `BoxedChildrenFn`

> **Note**: Components that need to iterate through children often use these types (e.g., `ChildrenFragment`), so you won’t be able to leverage attribute passthrough there.

---

#### **Solution: Use TypedChildren for Attribute Passthrough**

**Works as Intended ✅**

```rust
#[component]
#[allow(non_snake_case)]
pub fn Component(children: TypedChildren<impl IntoView + 'static>) -> impl IntoView {
    view! {
        {children.into_inner()()}
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Root() -> impl IntoView {
    view! {
        <Component attr:class="foo"> // or <Component {..} class="foo">
            <div/>
        </Component>
    } // Renders as <div class="foo"/>
}
```

_By using `TypedChildren`, the `class="foo"` attribute successfully passes through to the `<div/>` child._

##### Children Types That **Do** Support Passthrough

- `TypedChildren`
- `TypedChildrenMut`
- `TypedChildrenFn`
- `TypedChildrenFnMut`

> **Tip**: Where possible, rely on these types if you need attribute passthrough to flow from the parent component to its children.

### Handling Attributes with `Show`

When you need to conditionally render one of two possible views, the `<Show>` component is often the go-to tool in Leptos. However, it’s important to be aware of how attribute passthrough behaves under the hood. Specifically, **the fallback branch of `<Show>` uses `AnyView` internally**, which means attribute passthrough **only applies to the `<Show>`’s children**—not the fallback. The example below illustrates this:

```rust
#[component]
pub fn Root() -> impl IntoView {
    // ...
    view! {
        <Show
            when=move || show.get()
            fallback=span()  // or move || view! { <span/> }
            attr:class="foo" // or {..} class="foo"
        >
            <div/>
        </Show>
    }
    // show=true  -> <div class="foo"/>  ✅ As intended
    // show=false -> <span/>             ❌ Not as intended
}
```

Because the fallback uses `AnyView`, the `class="foo"` attribute is never applied when the `fallback` view renders.

#### Workaround: Create a Typed Variant of `Show`

A straightforward solution is to write your own variant of `<Show>` that replaces the fallback’s `ViewFn` type with an additional generic, thereby allowing attribute passthrough on both branches. One downside is that the fallback can no longer be optional—by design, the generic fallback can’t be inferred or defaulted when it’s a function.

Below is an example using a published crate that implements this pattern:

```rust
// [dependencies]
// leptos-typed-fallback-show = "0.0.3"
use leptos_typed_fallback_show::TypedFallbackShow;

#[component]
pub fn Root() -> impl IntoView {
    // ...
    view! {
        <TypedFallbackShow
            when=move || show.get()
            fallback=span()  // or move || view! { <span/> }
            attr:class="foo" // or {..} class="foo"
        >
            <div/>
        </TypedFallbackShow>
    }
    // show=true  -> <div class="foo"/>   ✅ As intended
    // show=false -> <span class="foo"/>  ✅ As intended
}
```

> **Note**: This approach may be deprecated or rendered unnecessary in the future if/when Leptos provides a built-in solution for applying `AddAnyAttr` to `AnyView`. Until then, tools like `TypedFallbackShow` can help you ensure attribute passthrough works consistently for both the main content and the fallback.

## Static & Dynamic Class Attribute Precedence

The dichotamy between static and dynamic attributes is an important one when considering the extension of the Leptos environment with ports of existing UI libraries. Here we focus on the `class` attribute definition.

### Static Classes

At any time you may define `attr:class="my many classes"` or `<{..} class="my many classes"/>` but this static definition of the class will overwrite any and all static class definition lower down in the component tree _completely_.

```rust
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Component attr:class="foo"> // or <Component {..} class="foo">
            <div class="bar baz"/>
        </Component>
    } // Renders as <div class="foo"/>
}
```

There is nothing wrong with this design as it makes sense, quoting Greg's words: [this is how attributes work in the browser](https://discord.com/channels/1031524867910148188/1320397561315459202/1320459932130087034). However, it presents a challenge when we start wanting to merge classes with special logic (`tw_merge`) or simple concatenation (`format!("foo {}", bar)`) down the component tree. There are a few things we can do, and it covers almost all cases, but none of them will give you full control of the class merging, except for defining your own class prop in the parent component before then passing it off as a static class definition at some entry point in your tree.

---

### Dynamic Classes

In some scenarios, you may want to combine or “merge” classes at different levels—one component might add a dynamic class while another defines static classes. Leptos supports this via [Dynamic Class Attributes](https://book.leptos.dev/view/02_dynamic_attributes.html#dynamic-classes), which let you conditionally apply classes like `class:foo=true`.

Below are two approaches: **one where the parent sets the dynamic class** and the child uses static classes, and another where the parent sets static classes while the child attempts to add a dynamic class. The end result depends on **who overrides whom** in the final HTML.

---

#### 1. Parent-Defined Dynamic Class

```rust
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Component class:foo=true> // <-- dynamic class defined by parent
            <div class="bar baz"/> // <-- static classes defined by child
        </Component>
    }
    // Renders as <div class="bar baz foo"/>
}
```

Here, the dynamic `foo` class merges with the child’s static `bar baz`. The final output becomes `<div class="bar baz foo"/>`.

---

#### 2. Child-Defined Dynamic Class

```rust
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Component attr:class="bar baz"> // <-- static classes defined by parent
            <div class:foo=true/>        // <-- dynamic class defined by child
        </Component>
    }
    // Renders as <div class="bar baz"/>
}
```

In this case, the **parent** has already declared its static classes (`"bar baz"`), which **overwrite** the child’s attempt to add `"foo"` dynamically. As a result, the final `<div>` is rendered with just `"bar baz"`—the dynamic `"foo"` class is effectively lost at runtime.

> **Note**: This behavior stems from how Leptos treats a **parent’s static class** as the final word. If you want to ensure `"foo"` is retained, you’d need to define all classes (static + dynamic) in the same place (e.g., the parent), or rely on an alternative approach like merging strings before passing them down.

---

#### 3. When Dynamic Classes Can’t Be Parsed by the View Macro

In many cases, dynamic classes work as intended. However, if you have a class that is valid but cannot be parsed by the view macro (e.g., `foo-[qux]`), you’ll encounter issues similar to what I experienced. This is where the `{..}` syntax comes into play:

```rust
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <Component {..} class=("foo-[qux]", true)>
            <div class="bar baz"/>
        </Component>
    } // Renders as <div class="bar baz foo-[qux]"/>
}
```

---

#### 4. The 26-Attribute Limit & Double-Nested Components

It’s important to note that **each additional dynamic class attribute** counts against your **26-attribute budget** in Leptos 0.7.3. This limitation arises from the tuple-based implementation in Leptos (see [here](https://github.com/leptos-rs/leptos/blob/165911b2e676bd4302cdfe890610012c070d6d83/tachys/src/view/tuples.rs#L405-L408)). If you need **more** than 26 dynamic attributes, you have two main options:

1. **Submit a PR or Maintain a Fork**
   If you have a compelling use-case, you have several upstream options:

   - Modify the tuple limit in Leptos
   - Explore variadic emulation patterns (like the approach in [variadics](https://docs.rs/variadics/latest/variadics/)), though note the [discussion in design-internals](https://discord.com/channels/1031524867910148188/1031525141382959175/1326221132860227675) about potential compile time impacts
   - Or, if you're feeling ambitious, tackle variadic generics in Rust itself (good luck!)

2. **Double-Nested Components**  
   By splitting your attributes across two levels, you can effectively bypass the 26-class limit on a single component. For example:

   ```rust
    #[component]
    pub fn Root() -> impl IntoView {
        // Let C = {c₁, c₂, ..., c₂₆} be a set of boolean class attributes
        view! {
            <Component
                class:c1=true
                // ... where |C| = 26
                class:c26=true
                // class:c27=true // ❌ Error: |C| > 26 exceeds trait bound
            >
                // However, let D = {c₂₇} be a new set where |D| = 1
                <Component class:c27=true>
                    <div>
                        // Then this div receives C ∪ D, where |C ∪ D| = 27
                        "This div has classes C ∪ D = {c₁,...,c₂₇}"
                    </div>
                </Component>
            </Component>
        }
    }
   ```

   Here, the **outer** `ComponentPasses` applies classes `c1` through `c26`, while the **inner** `ComponentPasses` adds `c27`. Each level stays within the 26-class limit, but collectively you apply 27 dynamic classes to the final `<div>`.

By leveraging nested components or organizing your classes carefully, you can overcome the 26-class restriction without modifying the Leptos source.

---

#### 5. Dynamic Class Array Syntax

While you might be familiar with the [Dynamic Classes documentation](https://book.leptos.dev/view/02_dynamic_attributes.html#dynamic-classes) and its support for setting multiple classes via a slice/array syntax, there's an important distinction in where this syntax can be used:

```rust
#[component]
pub fn Root() -> impl IntoView {
    view! {
        // ❌ Does not work on components
        <Component {..} class=(["foo", "qux", "quux"], true)>
            <div class="bar baz"/>
        </Component>
    } // Compile error
}
```

However, this same syntax works perfectly when applied directly to DOM elements:

```rust
#[component]
pub fn Root() -> impl IntoView {
    view! {
        // ✅ Works on DOM elements
        <Component {..} class="bar baz">
            <div class=(["foo", "qux", "quux"], true)/>
        </Component>
    } // Renders as <div class="bar baz foo qux quux"/>
}
```

## Using `AttributeInterceptor` (a.k.a. “Spread Props”)

For UI elements like **tables** (or buttons, inputs, etc.), you sometimes want to accept a bundle of unknown attributes and spread them onto a single DOM element—much like React’s `{...props}` pattern. Leptos provides an `AttributeInterceptor` component that captures all attributes passed into it, allowing you to reassign them exactly where you want:

```rust
use leptos::prelude::*;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::html;

#[component]
#[allow(non_snake_case)]
pub fn Table<C: IntoView + 'static>(
    // Typed children that support attribute passthrough
    children: TypedChildrenFn<C>,
    // Optional node ref for the <table>
    #[prop(into, optional)] node_ref: NodeRef<html::Table>,
) -> impl IntoView {
    // Store the children
    let children = StoredValue::new(children.into_inner());

    view! {
        <AttributeInterceptor let:attr>
            <div class=(["relative", "w-full", "overflow-auto"], true)>
                <table
                    node_ref=node_ref
                    class=(["w-full", "caption-bottom", "text-sm"], true)
                    {..attr} // <-- “spread” captured attributes onto the table
                >
                    {children.with_value(|children| children())}
                </table>
            </div>
        </AttributeInterceptor>
    }
}
```

In this example, any attribute you pass to `<Table>`—like `attr:class="some-other-classes"`, `attr:data-` attributes, `attr:id`, etc.—will be intercepted and then applied directly to the `<table>` element. This approach parallels the following **React** snippet from shadcn/ui, where unknown props flow into the table component itself:

```tsx
const Table = React.forwardRef<
  HTMLTableElement,
  React.HTMLAttributes<HTMLTableElement>
>(({ className, ...props }, ref) => (
  <div className="relative w-full overflow-auto">
    <table
      ref={ref}
      className={cn("w-full caption-bottom text-sm", className)}
      {...props} // “Spread” all remaining props
    />
  </div>
));
Table.displayName = "Table";
```

Leptos’s `AttributeInterceptor` simply performs a similar job: collecting all attributes (including `class`) so you can explicitly decide where they land. This pattern is useful whenever you need to wrap an element (like a `<table>`) in a container `<div>` but still give consumers the ability to pass attributes that apply only to the inner DOM node.

## Custom Class Merging Logic

Leptos does not support any custom class merging once you define static or dynamic classes at arbitrary points in the tree. Nor is there an official workaround for something like [tailwind-fuse](https://github.com/gaucho-labs/tailwind-fuse) to be used internally be Leptos, beyond the constraints discussed above.

## Conclusion & Recommendations

1. **Typed Children**: Use `TypedChildren`-based types whenever you need attribute passthrough to function properly.
2. **Fallback Branches**: `<Show>` and similar constructs use `AnyView` for fallbacks, so consider something like `TypedFallbackShow` to preserve attributes.
3. **Static vs. Dynamic**: If a static class is defined “above” a child, it can overwrite any dynamic additions below it. **Choose** where you define your classes carefully to ensure merging behaves as you expect.
4. **Class Macro Limits**: The 26-attribute limit can be bypassed with nested components, or you can submit a PR/fork.
5. **Dynamic Class Array Syntax**: `(["foo", "qux"], true)` is restricted to DOM elements, not custom components.
6. **No Automatic Merge**: If you want specialized merging (e.g., `tw_merge`), do it in a single location—Leptos won’t unify classes automatically once they’re defined in separate spots. Leptos appends dynamic classes in the order they appear as you go down the component tree.

By applying these guidelines, you can avoid unexpected overwrites, parse errors, or class-limit issues, and continue to build efficient, elegant UIs with Leptos.
