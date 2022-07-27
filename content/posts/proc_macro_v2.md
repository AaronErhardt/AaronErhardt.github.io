---
title: "Does Rust need proc-macros 2.0?"
date: 2022-07-27
# weight: 1
author: "Aaron Erhardt"
showToc: false
TocOpen: false
draft: false
hidemeta: false
comments: false
disableShare: false
disableHLJS: false
hideSummary: false
ShowReadingTime: true
ShowBreadCrumbs: true
ShowPostNavLinks: true
editPost:
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/tree/blog/content"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

Without any doubt, macros are an important feature of the Rust programming language.
Macros like `println!`, `lazy_static!`, various derive-macros and many others have saved countless hours of writing tedious boilerplate code.

However, not everything is perfect yet.
You might have heard about the [declarative macros 2.0](https://github.com/rust-lang/rust/issues/39412) effort to fix some longstanding issues with `macro_rules!`.
The new macro syntax is still WIP, but I hope it will be stabilized soon.

However, that just improves declarative macros, but Rust also has proc-macros (procedural macros).

## Proc-macros are awesome

> I'm not going to explain proc-macros in this article, but you can find more information [here](https://doc.rust-lang.org/reference/procedural-macros.html).

First, let's look at what makes proc-macros such a valuable feature for Rust.
Proc-macros are essentially extensions for the compiler that turn token streams into token streams.
In between, you can use regular Rust code and arbitrarily complex logic to reorder, create, remove or transform tokens.
There are hardly any limitations.
This makes proc-macros incredibly powerful and useful in situations declarative macros can't handle.

## The problems

Proc-macros are powerful, but they are also very difficult to implement correctly.
It's up to the individual developer to parse the macro syntax and to handle every edge-case.
At the same time, language servers struggle a lot to provide decent IDE integration for proc-macros because they have no idea how the macro works internally.

Let's have a closer look at some of those problems.

### A black box

The following code is an example of an imaginary `#[async_runtime::main]` attribute macro:

```rust
#[async_runtime::main]
async fn main() {
    log::info!("Starting application! 🚀🚀🚀");
    start_server().await;
}
```

Obviously, the `#[async_runtime::main]` macro just creates some boilerplate to start an async function, right? 
Actually, we don't know and neither does the compiler until it calls the proc-macro.
Unless you analyze the shared library or the source code of the proc-macro, you'll never know what code will be generated until you try it out.
And even then, you don't know whether the code generation will completely change as soon as you type another letter.

In our example, this could have been a valid expansion of the macro:

```rust
fn main() {
    open_link("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
}
```

To be fair, macros tend to produce rather predictable code in order to be usable.
Yet, language servers suffer a lot more from this problem.

For now, language server try to solve this problem by guessing that if something looks familiar, the macro probably does nothing unexpected.
Only if the output looks similar to the input and to a known Rust pattern, language servers enable auto-completion and other cool features.
However, this is still fragile because you never know what's happening inside of the macro.
More importantly though, language servers will probably never do anything more than error reporting for proc-macros that look only slightly different from valid Rust code. 

### Sandboxing

I'm sure that by now many Rust programmers are aware that proc-macros can execute arbitrary code.
Yet, there's still no sandboxing by default.
Ideally, Rust would sandbox proc-macros and also the execution of the binary which could contain malicious macro-generated code too.
For example, proc-macros shouldn't be able to access the file system or the internet by default.
In reality, this is easier said than done, but in my opinion, it's only a matter of time until the lack of sandboxing will be exploited.

### Hygiene

Proc-macros are unhygienic, which means that they are affected by the context they are called from.
To avoid naming conflicts, it's recommended to use absolute paths for imports (e.g. `::std::thread::spawn`).
This works great as long as the required crates can be imported with absolute paths.
However, once another crate re-exports a required crate, a user would usually just import the top-level crate and use its re-export.
Yet, this changes the absolute path of the crate, which in turn breaks the macro.

Ideally, Rust should be able to recognize paths that are independent from external items and should be evaluated in the context of "macro dependencies".
That wouldn't make macros entirely hygienic but certainly less error-prone.

## Proc-macro 2.0?

The compiler team has been aware of issues regarding sandboxing and macro hygiene for a long time now and I hope they will be addressed soon.
However, IDE integration and complex code parsing have been two longstanding issues that were only addressed partially.

I think it's time to solve these issues properly.

### A declarative parsing description

The [syn](https://docs.rs/syn/latest/syn/index.html) crate and more recently [venial](https://docs.rs/venial/latest/venial/) try to improve macro parsing by defining some structures and functionality to parse them.
Both crates have shown that it is possible to represent macro syntax with Rust's data structures.
However, the compiler still doesn't understand what's going on internally and custom syntax needs manual parsing logic.

Rust's syntax is documented in the [Rust reference](https://doc.rust-lang.org/reference/notation.html) with its own notation.
This implies that it is possible to specify Rust's syntax and probably also the syntax of most macros in a simple format.

Combining both ideas has huge potential: A simple notation that can be represented by Rust's data structures and read by machines as syntax description would solve both problems.
This notation could be its own lightweight language or just Rust data structures.

For examples, the compiler could expose some basic parsable types (e.g. structs or trait impls) and derive parsing definitions for custom types.
If all types used by the macro have to let the compiler derive their parsing logic, it would be easy for language servers and the compiler itself to understand the syntax of the macro.
All you need to do then is to tell the Rust compiler where to start parsing and it will return the Rust types corresponding to your syntax description.

<table>
<tr>
  <th>Rust reference grammar description</th>
  <th>Derived parsing definition</th>
</tr>
<tr>
<td>

```
 


StructField :
   OuterAttribute*
   Visibility?
   IDENTIFIER : Type


 
```

</td>
<td>

```rust
// Not just for parsing, also defines a
// syntax description.
#[derive(Parse)]
struct StructField {
   outer_attr: Vec<OuterAttribute>,
   vis: Option<Visibility>,
   ident: Identifier,
   _colon: Colon,
   ty: Type
}
```

</td>
</tr>
</table>

> Example for describing the syntax of a struct field

The biggest advantage of this approach is that it allows language servers to understand the syntax.
For example, they would know that the macro is looking for code that's a subset of Rust's struct definition.
This gives them much more opportunities to improve IDE integration.

### Opaque tokens

Another improvement for IDE integration would be a concept I call "opaque tokens".
When a macro doesn't need to access the value of a token because it doesn't influence the code generation, it could just as well tell the compiler that this token should be opaque.
Opaque tokens would be impossible to access or modify.
This gives language servers the information that a certain token is left unchanged to optimize performance while the user is typing and potentially enable even better IDE integration.

# Conclusion

Proc-macros are great as they are, but they have so much more potential.
Rust's third pillar - productivity - will surely profit from easier proc-macros and better IDE integration.

I hope this article brings up more ideas and discussions to improve the status quo of proc-macros.
Regardless of which solution will be chosen in the end, I believe that proc-macros deserve an upgrade.

**Feel free to [join the discussion](https://github.com/AaronErhardt/AaronErhardt.github.io/discussions/63) on GitHub!**

# Appendix: A larger code example

Format description

```noh
CustomStructStruct :
   struct IDENTIFIER  GenericParams? WhereClause? ( { StructFields? } | ; )

CustomStructFields :
   CustomStructField (, CustomStructField)* ,?

CustomStructField :
   OuterAttribute*
   CustomVisibility?
   IDENTIFIER : Type

CustomVisibility :
      pub
   | pub ( crate )
```

Rust code mockup

```rust
// Import existing types from the compiler
use proc_macro::*;

#[derive(Parse)]
#[subset_of(Struct)]
struct CustomStruct {
    struct_: Struct,
    name: Identifier,
    generics: Option<GenericParams>,
    where_clause: Option<WhereClause>,
    inner: Either<SemiColon, Backets<Option<CustomStructFields>>>,
}

#[derive(Parse)]
#[subset_of(StructFields)]
struct CustomStructFields {
    // We don't want to touch the struct field
    // so we wrap it into `Opque<T>`.
    fields: Punctuated<Opaque<CustomStructField>>,
}

#[derive(Parse)]
#[subset_of(StructField)]
struct CustomStructField {
    outer_attr: Vec<OuterAttribute>,
    vis: Option<CustomVisibility>,
    ident: Identifier,
    _colon: Colon,
    ty: Type
}

#[derive(Parse)]
#[subset_of(Visibility)]
enum CustomVisibility {
   Pub(Pub),
   PubCrate((Pub, Crate)),
}
```
