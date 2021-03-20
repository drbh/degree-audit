# Degree Audit

The degree audit service takes in two inputs and returns a report. The two inputs are a student's educational history and a task map.

The point of a degree audit is to evaluate a students history against a task map which is a set of requirements.

A student is eligiable for the degree if their classes fulfill all of the requirements.

## Usage

```bash
git clone git@github.com:drbh/degree-audit.git
```

```
cd degree-audit

# grab some ☕️, this takes ~3 mins
cargo build --release
```

## Run Degree Audit Server

```bash
./target/release/serve
# Degree Audit Server is running at 127.0.0.1:8080
```

## Important concepts

A student is modeled like

```rust
{
  "student": {
    "name": "drbh",
    "majors": ["beekeeping", "algorithimic trading"],
    "classes": [
      // this is a class experience
      {
        "when": 0,
        "grade": 0,
        // this is a class
        "class": {
          "hours": 10,
          "subject": "SCI",
          "level": 100,
          "group": ["MA", "C", "Z"]
        }
      }
    ]
  }
}
```

A task map is modeled like

```rust
{
  "map": [
    // this is a list of requirements
    [
      // this is a requirement
      {
        "original": "Mathematics (MA)",
        // requirements have cards
        "card": [
          // this is the first statement
          [
            // this is a brick
            {
              "match_type": "Group",
              "group": "Z"
            }
          ]
        ]
      }
    ]
  ]
}
```

The relationship between all of the requirements are a soft `AND` meaning that all of the requirements (1 AND 2 AND ...) need to be completed before the student fulfills the map. However it's "soft" since the program does not evaulate the rule - it's a rule that humans use.

On the other hand, there are hard rules that are dictated by the [logicmap](https://github.com/drbh/logicmap) concept.

This primative states that all Statements have an `AND` relationship and all Bricks have an `OR` relationship.

Using the simple logic we can model any complex combinations of items that fulfill a requirement.

Once the data is modeled in the above form - we can run the `report` methon on our student/map and return a comprehensive audit.

### Match types

We mentioned that the logicmap is doing most of the work - however in order to interface with logicmaps we need to define our match types. A match type refers to how any why something the student has completed "matches" with an item in the map.

We have simple types to start with for now.

- Exact Match (the same subject and level)
- Subject Match (the same subject)
- Group Match (class contains the group type)

however as more match types are uncovered they're easily added by adding a new match `struct` and `expression` like [the exact match example](src/exact.rs)

## Understanding the evaluation

Finally now that we've covered how this all works. Lets look at some results

```rust
[
  {
    "met_flag": true, // this is the card level
    "stmts": [
      {
        "title": "Statement 0",
        "met_flag": true, // this is the statement level
        "exp": [
          {
            "descr": "Group - Z",
            "path": ["classes"], // the path to what trigged this brick
            "index": [0], // this is the index of the path
            "met_flag": true // this is the brick level
          }
        ]
      }
    ]
  }
]
```
