# Typstyle Playground

An interactive playground for the [Typstyle](https://github.com/enter-tainer/typstyle) formatter, built with React, TypeScript, and Vite.

## 🚀 Features

- **Two-Panel Editor**: Source code editor (left) and formatted output viewer (right)
- **Multiple Output Views**: Switch between formatted code, AST visualization, and Pretty IR views
- **Real-time Formatting**: Reactive formatting without manual button clicks
- **Customizable Options**: Configure formatting settings (indent size, line length, etc.)
- **Professional Editor**: Monaco Editor with custom Typst syntax highlighting
- **Modern UI**: Clean, responsive interface with light/dark theme support
- **Frosted Glass Design**: Beautiful modern UI with backdrop blur effects

## 🛠️ Tech Stack

- **Frontend**: React 19 + TypeScript
- **Build Tool**: Vite 6.x
- **Package Manager**: Bun
- **Styling**: TailwindCSS 4.x + SCSS
- **Code Editor**: Monaco Editor with custom Typst language support
- **Linting**: ESLint with TypeScript support

## 🏗️ Project Structure

```text
playground/
├── public/
│   ├── favicon.svg              # Custom Typstyle favicon
│   └── apple-touch-icon.png     # Apple touch icon
├── src/
│   ├── App.tsx                  # Main application component
│   ├── index.scss               # Main stylesheet that imports all modular CSS
│   ├── main.tsx                 # React app entry point
│   ├── typst-language.ts        # Monaco Editor Typst language definition
│   ├── vite-env.d.ts           # Vite environment types
│   └── styles/                 # Modular SCSS files for better organization
│       ├── _reset.scss         # CSS reset and base HTML styles
│       ├── _themes.scss        # CSS custom properties for theming
│       ├── _base.scss          # Base app styles, header, and layout
│       ├── _components.scss    # Glass panels, buttons, and UI components
│       ├── _forms.scss         # Form inputs, checkboxes, and labels
│       ├── _layouts.scss       # Responsive layout styles (wide/middle/thin)
│       └── _monaco.scss        # Monaco Editor specific styles
├── package.json                 # Project dependencies and scripts
├── tailwind.config.mjs          # TailwindCSS configuration
├── vite.config.ts              # Vite build configuration
└── README.md                   # This file
```

## 🚦 Getting Started

1. **Install dependencies:**

   ```bash
   bun install
   ```

2. **Start development server:**

   ```bash
   bun run dev
   ```

3. **Build for production:**

   ```bash
   bun run build
   ```

4. **Preview production build:**

   ```bash
   bun run preview
   ```

## 🎨 Theme Support

The playground features a comprehensive theming system:

- **Light Theme**: Green/mint color palette inspired by Komeiji Koishi
- **Dark Theme**: Dark blue-teal color palette
- **CSS Custom Properties**: All theming managed through CSS variables
- **Automatic Monaco Editor Integration**: Themes automatically applied to code editors

## 🏗️ Architecture & Recent Improvements

### Modular CSS Structure

The project uses a well-organized modular CSS architecture:

- **`src/styles/_reset.scss`**: CSS reset and base HTML element styles
- **`src/styles/_themes.scss`**: CSS custom properties for theming (light/dark modes)
- **`src/styles/_base.scss`**: Base application styles, header, and layout foundations
- **`src/styles/_components.scss`**: Glass panels, buttons, and UI component styles
- **`src/styles/_forms.scss`**: Form inputs, checkboxes, labels, and form controls
- **`src/styles/_layouts.scss`**: Responsive layout styles for wide/middle/thin breakpoints
- **`src/styles/_monaco.scss`**: Monaco Editor specific styling and customizations

### Unified Editor Architecture

The playground now features a unified editor system with a single `UnifiedEditor` component that handles both code editing and output display:

- **Single Source of Truth**: One `UnifiedEditor` component replaces multiple editor implementations
- **Configuration-Based Behavior**: Editor behavior (readonly, language, features) controlled via props
- **Consistent Styling**: All editors share the same Monaco Editor options and theming
- **Reduced Code Duplication**: Eliminated repetitive editor configuration across components

**Editor Configurations by Use Case:**

- **Source Code Editor**: `typst` language, editable, line numbers, folding, word wrap
- **Formatted Output**: `typst` language, readonly, no line numbers, no folding
- **AST/IR Output**: `json` language, readonly, no line numbers, folding enabled

### Editor State Management

- **Shared Editor Instances**: Single Monaco Editor instances shared across responsive layouts
- **State Preservation**: Cursor position, scroll state, and undo history persist across layout changes
- **Centralized Management**: `EditorManager` component handles editor lifecycle and configuration
- **Responsive Design**: Seamless editor experience across wide (3-column), middle (2-column), and thin (tabbed) layouts

## 🔧 Format Options

Customize the formatting behavior:

- **Indent Size**: 1-8 spaces (default: 2)
- **Max Line Length**: 40-200 characters (default: 80)
- **Insert Final Newline**: Add newline at end of file
- **Trim Trailing Whitespace**: Remove trailing spaces

## 🚧 Future Enhancements

- [ ] Integrate actual Typstyle WASM module or API
- [ ] Enhanced Typst syntax highlighting
- [ ] Real AST and IR parsing
- [ ] Export functionality for formatted code
- [ ] File import/export capabilities
- [ ] Keyboard shortcuts for common actions

## ✅ Recent Improvements

- ✅ **Modular CSS Architecture**: Split monolithic CSS into organized SCSS modules for better maintainability
- ✅ **Responsive Layout System**: Three responsive breakpoints (wide/middle/thin) with optimized layouts
- ✅ **Editor State Persistence**: Monaco editors preserve cursor position, scroll state, and undo history across layout changes
- ✅ **Enhanced Monaco Integration**: Full Monaco Editor integration for all output views with proper syntax highlighting

## 📄 License

This project is licensed under the Apache-2.0 License - see the [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request to the main [Typstyle repository](https://github.com/enter-tainer/typstyle).
