### 🎯 The Big Picture

This JSON file is a detailed, machine-readable report of your project's test coverage. It tells you exactly which lines of your code were run during your tests and which were not. It's structured like a tree, starting from the whole project (`totals`), branching into individual files, and then further into the classes and functions within those files.

-----

### 🔑 Key and Value Conventions (The Important Stuff\!)

#### 1\. Resource Location: File Paths are Keys

The most important convention is how files are identified. The keys inside the main **`"files"`** object are the **relative paths** to your source code files from the root of where you ran the coverage command.

  * **Example**: `"my_project/main.py": { ... }`
  * **Why it matters**: This is how the tool maps the coverage data back to your actual code. When you build tools to parse this file, you'll use these keys to know which file the data belongs to.

#### 2\. The Recursive Russian Doll 🪆

The schema is **recursive**. A `file` object contains `classes` and `functions` objects. A `class` object can *also* contain `classes` (for nested classes) and `functions` (for its methods). This pattern repeats, perfectly mirroring your code's structure. This is handled by the `"$ref": "#/$defs/coverageBlock"` in the schema, which is like saying "this part of the structure can contain a smaller version of itself."

#### 3\. The Quirky Negative Line Numbers in Branches

This one is a "gotcha" but it's super important. In the **`"executed_branches"`** and **`"missing_branches"`** arrays, you'll see pairs of numbers like `[28, 29]` or `[28, -19]`.

  * `[28, 29]`: This means a branch was taken from line `28` to line `29`. Simple\!
  * `[28, -19]`: This means a branch was taken from line `28` to the **exit of the code block** that started on line `19`.
  * **Why a negative number?**: It's a clever way to show a jump *out* of a block (like a `return` or the end of a function) without needing to know the exact exit line number, which can be ambiguous. **Do not mistake this for an error\!** It's intentional and crucial for accurate branch analysis.

#### 4\. Context is Everything: The `"contexts"` Object

The **`"contexts"`** object tells you *why* a line was executed.

  * **Key**: The line number, but as a **string** (e.g., `"65"`).
  * **Value**: An array of strings. Each string is the name of a test that caused that line to be executed (e.g., `"tests.test_main.test_analytics_processing"`).
  * **Quirk**: Sometimes the context is an empty string `""`. This usually means the line was executed at the module level (e.g., an import statement) before any specific test function was called.

#### 5\. Dual Percentages: `percent_covered` vs. `percent_covered_display`

In every **`"summary"`** block, you'll see two percentage fields.

  * **`"percent_covered"`** (number): This is the raw, high-precision floating-point value (e.g., `84.615384...`). Use this for any mathematical calculations.
  * **`"percent_covered_display"`** (string): This is a nicely formatted string, usually rounded to two decimal places (e.g., `"84.62"`). Use this for displaying in reports or UIs.

By understanding these key conventions, you'll be able to confidently parse and utilize any `coverage.json` report. Happy coding\!
