# Introduction to Nenyr

Nenyr is the declarative domain-specific language (DSL) that powers the **Galadriel CSS** framework. This framework is designed to streamline the process of creating advanced, modular, and context-aware styles for web applications. Nenyr provides a structured, intuitive approach to styling, ensuring scalability, flexibility, and maintainability. By following Nenyr’s syntax and grammar rules, developers can create responsive, theme-based designs with minimal effort, all while adhering to best practices for modular CSS development.

## Core Principles of Nenyr

Nenyr is based on several key principles that enable developers to write clear, maintainable, and efficient styles. These principles form the backbone of how styles are constructed within the **Galadriel CSS** framework:

### 1. **Declarative Syntax**

Nenyr uses a declarative syntax, making it both human-readable and machine-optimized. This ensures that styles are concise, self-explanatory, and easy to maintain. Within a block of Nenyr code, method declarations are separated by commas, and optional trailing commas are allowed for flexibility.

### 2. **Modular Styling**

Nenyr promotes modularity by organizing styles into distinct contexts. Each context represents a different scope of styling—**Central**, **Layout**, and **Module**—ensuring that styles are applied only where necessary. A key rule to follow when using Nenyr is that **each `.nyr` file should contain only one context construction**. Defining multiple contexts within the same file will trigger an error during the build process, as this is forbidden in the framework.

### 3. **Utility-First Design**

**Galadriel CSS** is built on a **utility-first design** philosophy, ensuring that each Nenyr property-value pair generates a unique CSS utility class. If you declare the same property-value combination multiple times across your code, **Galadriel CSS** will optimize this by generating only a single CSS utility class for that declaration. This approach minimizes redundancy and maximizes performance, as identical declarations are merged into a single, reusable class. By writing clean, context-aware Nenyr code, developers delegate the complex tasks of compiling and optimizing CSS to the **Galadriel CSS** framework during the build process. As a result, the framework efficiently handles style generation and minimizes the final CSS file size, ensuring a lean, maintainable, and scalable application.

---

## Key Features of Nenyr

Nenyr offers a wide range of features that make it a powerful tool for styling web applications. Below are some of the most important features:

### 1. **Imports**

Nenyr allows you to import external CSS files or third-party styles, seamlessly importing them into the output CSS content. This ensures that external resources can be easily incorporated into your project.

### 2. **Typefaces**

Custom fonts can be declared and used consistently across your application, ensuring a cohesive and distinctive typographic style.

### 3. **Breakpoints**

Breakpoints provide the ability to create responsive designs. Once defined, they can be consistently applied throughout the application, ensuring your design adapts smoothly to different screen sizes.

### 4. **Themes**

Nenyr allows you to define themes that manage light and dark modes. These themes ensure consistency across your application and adapt to the user's preferences.

### 5. **Aliases**

To simplify the code and make it more personalized, Nenyr allows the definition of aliases for common properties. This improves readability and reduces repetition.

### 6. **Variables**

Reusable values, such as colors or dimensions, can be declared as variables. This promotes consistency and ease of maintenance across your stylesheets.

### 7. **Animations**

Nenyr supports the creation of complex animations through declarative methods. You can define animations at various states (e.g., `From`, `Halfway`, `To`, `Fraction`, or `Progressive`), with progressive transformations to create intricate effects.

### 8. **Classes**

Classes are central to Nenyr. Styles are encapsulated within classes that can inherit from other classes, promoting a hierarchical approach to styling. Additionally, classes support primary styles, pseudo-methods like `Hover` and `Active`, and conditional styles based on breakpoints.

---

## Nenyr Syntax and Grammar Rules

### General Structure

Nenyr follows a structured syntax for defining various styling elements. Here's an overview of the core grammar rules:

-   **Method Declarations**: Begin with the `Declare` keyword, followed by the method type (e.g., `Imports`, `Variables`, `Animation`).
-   **Comma-Separated Elements**: Elements within each declaration are separated by commas, with an optional trailing comma for flexibility.
-   **Context Structure**: **Only one context construction is allowed per `.nyr` file**. Defining more than one context within a file will cause an error. Ensure each `.nyr` file adheres to this rule for proper compilation.

### Comments

-   **Line Comments**: Use `//` to add single-line comments in your code.
-   **Block Comments**: Use `/* */` for multi-line comments.

### Value Placeholders

-   **Variables or Animations**: Reference variables or animations by wrapping their name in `${}` (e.g., `backgroundColor: "${primaryColor}"`, `animationName: "${myAnimationName}"`).
-   **Default Values**: Direct values can be used inline, without needing interpolation.

### Grammar Rules

-   **Case Sensitivity**: Keywords such as `Declare` and `Variables` are case-sensitive and must be used in the exact format.
-   **Valid Identifiers**: Identifiers must start with a letter and contain only alphanumeric characters.
-   **Duplicate Declarations**: If a declaration is made multiple times within the same scope, the most recent one will overwrite previous ones.
-   **Escaped Characters**: Special characters (e.g., quotes) must be escaped using sequences like `\"`.

---

## Compilation and Transformation

When writing Nenyr code in `.nyr` files, the **Galadriel CSS** framework processes this code and transforms it into optimized CSS during the build process. Here's how the compilation process works:

1. **Compilation**: The Nenyr code is parsed, processed and transformed into a series of CSS utility classes.
2. **Injection**: These utility classes are then injected into the entry point CSS during the build process.
3. **Output**: The final production-ready HTML contains the utility class names injected, while the associated CSS rules are injected into the application’s CSS entry point.

### Integration Workflow

The integration client handles the communication between the **Galadriel CSS** and the application’s front-end. It replaces the Nenyr markups with the appropriate CSS utility classes and ensures that the required CSS rules are applied globally.

---

## Best Practices for Nenyr

To make the most of Nenyr and maintain clean, scalable code, consider the following best practices:

1. **Comment Your Code**: Use comments to explain the purpose and structure of your code. Single-line comments can be created with `//`, while block comments use `/* */`.
2. **Optimize Animations**: Be mindful of performance when using animations. Use them sparingly and ensure they enhance the user experience without compromising performance, especially on mobile devices.
3. **Define Themes**: Use themes to define light, or dark modes. This will help maintain consistency across your design system.
4. **Stick to One Context Per File**: Ensure that each `.nyr` file contains only one context definition. Attempting to define more than one context in a single file will result in an error during the build process.
5. **Organize Styles into Contexts**: To keep your styles organized and maintainable, structure them into the appropriate contexts: **Central**, **Layout**, and **Module**. This helps to clarify the scope of each style and ensures better readability and scalability.

---

## Final Thoughts

Nenyr is the heart of **Galadriel CSS** (or let me say, Nenyr is the ring of power of **Galadriel CSS**), providing a powerful, declarative syntax that enables developers to create complex, modular styles. By adhering to the grammar rules and syntax of Nenyr, you unlock the full potential of scalable and context-aware styling. Its flexible, modular approach ensures that your styles remain maintainable as your application grows, making it an ideal choice for large, dynamic projects.

To maximize the power of Nenyr, be sure to follow the best practices outlined in this guide. Whether you're working with animations, themes, or breakpoints, Nenyr gives you the tools you need to craft efficient and adaptive styles.

Explore further documentation to dive deeper into specific Nenyr contexts and methods available for you to leverage in your projects!
