# Linear Regression Analysis: Candy Pricing Prediction

This document demonstrates the application of simple linear regression to predict candy pricing based on the number of candies.

```graphql
number of candy | price of candy
________________|_________________
        10      |        25 $
        20      |        50 $
        30      |        75 $
        47      |         ? (find the price)
```

```graphql

note: this graph is not accurate and is only used for showcasing example.

                  ^Y
                  |
                  |
                  |
                  |_ 75
                  |
                  |_ 50      // price of candy
                  |
                  |_ 25
                  |
 -----------------|---|---|---|------|-->  // number of candy
                  | X 10  20  30     47
                  |
                  |
                  |
                  |
                  |
                  |
                  |
                  |
```

#### Components of the Linear Equation:

The linear queation used in simple linear regression is: [ y = b_0 + b_1x ]

- y = Dependent variable (trying to predict the value, e.g. price of candy)
- x = Independent variable (the variable to make predictions, e.g. number of candies)
- (b_0) = y-intercept of the line (the value of (y) when (x) is 0 )
- (b_1) = Slope of the line (how much (y) changes for a one-unit change in (x))

#### Calculating the Slope (b_1):

The slope ((b_1)) of the line is a measure of how steep the line is. It is calculated as change in (y) divided by the change in (x) between two points on the line.

- Mathematically, it's expressed as: [ b_1 = \frac{\Delta y}{\Delta x} ] Where:
  - (\Delta y) is the difference in the (y) values of two data points.
  - (\Delta x) is the difference in the (x) values of the same two data points.

```math
[ b_1 = \frac{\Delta y}{\Delta x} ] // formula to use

Example calculation of the slope:

 point 1: (10,25)
 point 2: (20,50)
 point 3: (30,75)

- The change in (y) ((\Delta y)) from these points is: [\Delta y = 75 - 50 = 25]
- The change in (x) ((\Delta xy)) is: [\Deltax = 30 - 20 = 10]

So, the slope ((b_1)) is: [b_1 = \frac{25}{10} = 2.5]

This means that for each candy added, the price increases by 2.5
```

#### Calculating the Intercept (b_0):

The intercept (b_0) is where the line crosses the (y)-axis. To find (b_0), you use the equation of the line and solve for (b_0) when (x) and (y) are known from a data point.

```math
Using point 1: (10,25)

[ y = b_0 + b_1x ] // using the formula with a known data point
[25 = b_0 + 2.5 * 10]
[25 = b_0 + 25]
[b_0 = 25 - 25 = 0]
```

### Using the equation for Prediction:

Now that we have both (b_0) and (b_1), you can use the equation to predict (y) price of the candy for any (x) number of candies:

```math
here,
b_0 = 0, b_1 = 2.5, x = 47

[ y = b_0 + b_1x ]
[y = 0 + 2.5 * 47]
[y = 117.5 ]
```
