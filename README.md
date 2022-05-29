<div align="center">
	<h1>identicode</h1>
	<p><i>code that identifies you</i></p>
	<a href="https://crates.io/crates/identicode"><img src="https://img.shields.io/crates/v/identicode?style=flat-square"/></a>
	<img src="https://img.shields.io/github/license/gulje/identicode?style=flat-square"/>
</div>

## Build & Installation
It can be downloaded and installed simply with the following command:
```sh
cargo install identicode
```

## How to use?
```
$	language mode
@	branch mode
?	os mode
&	others mode

+	add 1
*	add 5
/	add 10
%	add 50
=	add 100
-	substract 1

;	push language/branch/os by current value in stack 
```
When you use `;`, the string corresponding to the value in the stack is added whatever mode you are in.

For example, you want to add the Rust `(index: 203)` language.
- First, we have to enter our identicode version. Simply run `identicode` and it will give you your version. (ex. `+++;`)
- Then, we need to switch to language mode, because we want to add a language: `$`
- Write the code corresponding to the index: `==+++`
- After you write the code, you can add Rust to the languages section by using `;`.
- Final code: `+++;$==+++;`

You can also apply this process with operating systems (`?`), branches (`@`) and
others (`&`).

## Indexes:
If you can't find what you want, you can create a [Pull Request](https://github.com/gulje/identicode/pulls)!

All lists are present in `src/data` folder. Add what you want to add at the bottom.

An item's index is found by `(the line number it is in - 1)`.

`identicode` will always be backwards compatible, so an identicode that works correctly
in your version will still work correctly in the newest versions.

## License
`identicode` is licensed under the terms of [Apache License 2.0](LICENSE).
