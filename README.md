# computerv1
A simple program that solves a polynomial second or lower degree equation

## Usage
    git clone "..." ; cd computerzv
    cargo run [your equation]


-> every entity of the equation need to be formatted like **$$n \times X^y$$** like

##### $$5\times X^0+4\times X^1-9,3\times X^2=1\times X^0$$

## Solving a Second Degree Equation

A quadratic equation is given in the form:

##### $$ax^2 + bx + c = 0$$

where:
- $a \neq 0$ (if $a = 0$, it is not a quadratic equation),
- $b$ and $c$ are coefficients.

### 1. Calculating the Discriminant

The discriminant is calculated as:

##### $$Δ = b^2 - 4ac$$

- If $Δ > 0$, there are two distinct real solutions.
- If $Δ = 0$, there is one real solution (a double root).
- If $Δ < 0$, there are two complex conjugate solutions.

### 2. Formula for the Solutions

The solutions of the quadratic equation are given by:

##### $$x_1, x_2 = (-b ± \sqrt{Δ}) / (2a)$$

- If $Δ > 0$: The two solutions are real and distinct:
    
##### $$x_1 = (-b + \sqrt{Δ}) / (2a)$$
##### $$x_2 = (-b - \sqrt{Δ}) / (2a)$$
  
- If $Δ = 0$: The solution is real and repeated:

##### $$x = -b / (2a)$$
- If $Δ < 0$: The solutions are complex:
    
##### $$x₁ = (-b + i√|Δ|) / (2a)$$ $$x₂ = (-b - i√|Δ|) / (2a)$$

## Special Cases in Solving Equations

### Special Case 1: Infinite Solutions

An equation can have infinitely many solutions when the equality holds true for any value of the variable.

#### Example:


##### $$4x + 1 + x = 3 + 5x - 2$$


Simplify both sides:


##### $$4x + x - 5x = 3 - 2 - 1$$
##### $$0 = 0$$


The resulting equality, $$0 = 0$$, is always true regardless of the value of $(x)$.  
Thus, the equation has **infinitely many solutions**.

---

### Special Case 2: No Solution

An equation may have no solution when the equality is false for any value of the variable.

#### Example:


##### $$3x + 2 = 2x + 5 + x$$


Simplify both sides:


##### $$3x - 2x - x = 5 - 2$$

##### $$0 = 3$$


The resulting equality, $$0 = 3$$ , is false.  
Thus, the equation has **no solution**.
