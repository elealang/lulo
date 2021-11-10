<p align="center">
<img width=12.5% src="./media/logo.svg">
</p>

<br/>

<h1 align="center">Lulo: A Data Language</h1>


AN AMBITIOUS PROJECT
most programming languages INCLUDE the data structure langauge for you.
but what if we think about things orthogonally, so you can bring your own 
data langauge to your project.

zen of programming. to see data as it is, not as we want it to be.


# core data structures

## values

set, list, integer, text, etc...

values can be non-deterministic

do we need set/list types then?

## changes

change = operation over: set, list, integer, text, etc...

* 'add'
* 'in'

these are hard-coded so they can be optimized, maybe not complete
could be extended

## types

equation of value <op> value = value

### is text

(value must be text)
value + "" = * (nd "any" text value)

### is text "dog"

value + "" = "dog"

### is one of

value `in` other_values



## code style

one error type per module, defined by scope of package
use structure of interface, lib, main.rs, object, and types
types structs should all be public
