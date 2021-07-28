
data types

the ultimate IDL


schema - collection of types
type - product | sum | int | text | float | alias | symbol | set t | list t | uuid | date | timestamp

data
 - has type


parse data
 - use type, already loaded from schema
 - check that data matches prescribed type
   - optionally infer the type


start:
create schema schema

write "load schema" function with parse data for schema schema




AN AMBITIOUS PROJECT
most programming languages INCLUDE the data structure langauge for you.
but what if we think about things orthogonally, so you can bring your own 
data langauge to your project.


V1 is the version with everything hardcoded
V2 will use the parent directory for everything like it nromally would
   and will be where we use the generated Schema code!
