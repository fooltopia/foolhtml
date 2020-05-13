# FoolHTML
A indentation based html template for Rust inspired by [Slim](http://slim-lang.com/)

It uses indentation to determine the scope of elements.

Still in early development. Please don't use it in your projects.

A simple example:
```
h1#title.fancy.large Hello World
div
  img#title-image src="images/title.jpg" width=1000 height=300 alt="A great title image."
```
will be turned into
```html
<h1 id="title" class="fancy large">Hello World</h1>
<div>
  <img id="title-image" src="images/title.jpg" width="1000" height="300" alt="A great title image." />
 </div>
```
The rendered output does not contain any new lines and indentation, but a pretty-print mode for the renderer is planned. 

## Documentation
In FoolHTML, the indentation determines the scope of each element. Nested elements are added in the line below with a higher indentation level. *Indentation is two spaces*. This can not be configured at the moment. 

### The Templating Language
#### Elements
An element consists at least of a tag, like `<br />`, for example. The tag is the first word of a line. Optionally, you can add an id, classes, and attributes. These are explained below. 

After these, you can add the element's content. There are two ways to do it. 

A single line element:
```
h1 Hello World
```
renders to
```html
<h1>Hello World</h1>
```
A block element: A block is opened by putting a `:` at the end of the element's line. Then you can add the content in a new line with a higher indentation level. Each new line will be separated with a `<br />` during rendering. 
```
p:
  A block makes
  reading easier
  sometimes.
```
renders to 
```
<p>A block makes<br />reading easier<br />sometimes.</p>
```

#### Id and Classes 
You can add classes and ids directly after the tag and before the content. The id starts with `#` and the classes with `.`.
```
h1#title.big.fancy Hello World
```
renders to
```
<h1 id="title" class="big fancy">Hello World</h1>
```
#### Attributes
There are two ways to add attributes: Quoted and unquoted. 

Unquoted attribute values can only contain number, characters and `-`. For more complex attributes like urls, the quoted version is needed. 

Quoted attributes can use either double (`"`) or single (`'`) quotes. Then you can use the other type of quote inside the attribute value. Unless you use double quotes within the attribute value, the rendered result will always use double quotes.

Here's an example of an quoted and unquoted attributes
```
img.portrait url="images/anderson.jpg" alt='Thomas "Neo" Anderson' width=400
```
renders to
```
<img class="portrait" url="images/anderson.jpg" alt='Thomas "Neo" Anderson' width="400" />
```

### Using the Library
The library is not available on crates.io yet. To use it, you have to check it out from the repository. It provides a single public function:
```Rust
pub fn render_static_template(input: &str) -> String 
```
It takes your template string and returns the rendered string. 

## Future Plans
The next step is to add dynamic elements that can be rendered during run-time. Also parent and child templates will be added. 
