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
│   ├── index.scss               # Global styles with CSS custom properties
│   ├── main.tsx                 # React app entry point
│   ├── typst-language.ts        # Monaco Editor Typst language definition
│   └── vite-env.d.ts           # Vite environment types
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

## 📄 License

This project is licensed under the Apache-2.0 License - see the [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request to the main [Typstyle repository](https://github.com/enter-tainer/typstyle).
