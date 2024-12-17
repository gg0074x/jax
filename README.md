![alt text](assets/jax_logo.png?raw=true "Logo")

# ⚠️ JAX IS INCOMPLETE AND CURRENTLY A WORK IN PROGRESS

_It's missing many main features, code is messy, error handling is non-existant._

## ❓ What is JAX and how does it work?

Jax is a simplistic and minimal interpreted programming language with the mere purpose of serving as **a toy language** or **simple** modifiable "scripting language".   
In Jax, you define a set of functions named rules in a `DARULES!` file in the root directory of your code.  

### Rules follow this structure: 
```
RuleName : x < x + - >
```
Where the Rule name will always start with an uppercase letter and its parameters will be a single lowercase letter.
In between the `< >` we will insert our code with the following options:
- Calls to other rules or itself: `RuleName x`
- Add to a variable: `x +`
- Subtract to a variable: `x -`
- Loop a rule call `y` times: `RuleName x ? y`

Once we wrote our rules we can write our actual executing code in a separate `.jax` file knowing that:
- You can define variables in this file: `x = 1`
- Only available types are integers (for now)
- You can call the rules from your `DARULES!` file: `RuleName x`
- You cannot add or subtract to variables in this file
- You can loop rules just like in the rule code: `RuleName x ? y`

## ❓ Can i use it?

No, Jax is currently not usable since it's a work in progress project.  
Jax main purpose is to be an easily modifiable programming language, it is planned to be used as part of bigger projects or as a toy language where you would be able to make use of Jax's interpreter to implement easy and simple scripting.
