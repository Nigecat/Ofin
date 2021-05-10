This is mostly for internal purposes at the moment so I can keep track of what does what.
Do not expect this to be comprehensible and the contents of this may change at any moment.
Semicolons are omitted in single statements for readability.

## Variables
Variables do not need to be declared before being assigned.
`a = 5` would create a new variable named `a` and assign it the value `5`. 
Variables are also mutable, so 
```
a = 5;
a = 2;
```
would first assign `a` a value of `5` then assign it a value of `2`.

## Import
`import <file.ofin>`
Import the contents of another file, this takes a relative path to the file location of the caller (e.g `import <../otherfolder/file.ofin>` will go up a directory then into `otherfolder/file.ofin`). This is effectively a copy-paste import and any namespace collisions will cause an error.

## Using
`using <time>`
Bring everything from std::time into the current namespace. This makes all the functions in the specified module directly callable (there is no way to call module functions without importing their namespace).
