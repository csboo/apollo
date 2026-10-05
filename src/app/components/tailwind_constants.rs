// Design system constants - Clean modern aesthetic
// Uses CSS variables from tailwind.css

/// Primary action button - refined with subtle glow
pub const BUTTON: &str = "
    inline-flex items-center justify-center gap-2
    h-11 px-6
    rounded-lg
    bg-(--accent-primary) hover:bg-(--accent-primary-hover)
    text-white font-medium text-sm
    border-0
    shadow-md hover:shadow-lg hover:shadow-(--accent-primary)/20
    transition-all duration-200
    focus:outline-none focus:ring-2 focus:ring-(--accent-primary)/50 focus:ring-offset-2 focus:ring-offset-(--bg)
    active:scale-[0.98]
    disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100
";

/// Danger button - clear and intentional
pub const BUTTON_RED: &str = "
    inline-flex items-center justify-center gap-2
    h-10 px-5
    rounded-lg
    bg-(--accent-danger) hover:bg-(--accent-danger-hover)
    text-white font-medium text-sm
    border-0
    shadow-sm hover:shadow-md hover:shadow-(--accent-danger)/20
    transition-all duration-200
    focus:outline-none focus:ring-2 focus:ring-(--accent-danger)/50 focus:ring-offset-2 focus:ring-offset-(--bg)
    active:scale-[0.98]
";

// Secondary button - ghost style
// deprecated
// pub const BUTTON_SECONDARY: &str = "
//     inline-flex items-center justify-center gap-2
//     h-11 px-6
//     rounded-lg
//     bg-transparent hover:bg-(--bg-surface)
//     text-(--text-primary) font-medium text-sm
//     border border-(--border-default) hover:border-(--border-strong)
//     transition-all duration-200
//     focus:outline-none focus:ring-2 focus:ring-(--accent-primary)/50 focus:ring-offset-2 focus:ring-offset-(--bg)
//     active:scale-[0.98]
// ";

/// Text input - clean and functional
pub const INPUT: &str = "
    w-full max-w-[280px] h-11 px-4
    rounded-lg
    bg-(--bg-surface)
    text-(--text-primary) text-sm
    placeholder-(--text-muted)
    border border-(--border-default)
    hover:border-(--border-strong)
    focus:outline-none focus:border-(--accent-primary) focus:ring-1 focus:ring-(--accent-primary)/30
    transition-all duration-200
";

/// Select dropdown - matching input style
pub const SELECT: &str = "
    w-full max-w-[280px] h-11 px-4
    rounded-lg
    bg-(--bg-surface)
    text-(--text-primary) text-sm
    border border-(--border-default)
    hover:border-(--border-strong)
    focus:outline-none focus:border-(--accent-primary) focus:ring-1 focus:ring-(--accent-primary)/30
    transition-all duration-200
    appearance-none cursor-pointer
";

/// File input - subtle and modern
pub const CSV_INPUT: &str = "
    w-full px-4 py-3
    rounded-lg
    bg-(--bg-surface)
    text-(--text-secondary) text-sm
    border border-dashed border-(--border-default)
    hover:border-(--accent-primary)/50 hover:bg-(--bg-hover)
    focus:outline-none focus:border-(--accent-primary)
    transition-all duration-200
    cursor-pointer
    file:mr-4 file:py-2 file:px-4
    file:rounded-md file:border-0
    file:text-sm file:font-medium
    file:bg-(--accent-primary) file:text-white
    file:cursor-pointer file:transition-colors
    file:hover:bg-(--accent-primary-hover)
";

// Subtle shine effect - refined
// deprecated :(
// pub const FLASH: &str = "
//     relative overflow-hidden
//     before:pointer-events-none before:absolute
//     before:inset-0
//     before:bg-gradient-to-r before:from-transparent before:via-white/10 before:to-transparent
//     before:translate-x-[-200%]
//     before:transition-transform before:duration-700 before:ease-out
//     hover:before:translate-x-[200%]
// ";

// Card container
// deprecated
// pub const CARD: &str = "
//     p-6
//     rounded-xl
//     bg-(--bg-elevated)
//     border border-(--border-subtle)
//     shadow-lg
// ";

// Section title
// deprecated
// pub const SECTION_TITLE: &str = "
//     text-2xl font-semibold
//     text-(--text-primary)
//     tracking-tight
// ";

// Form label
// deprecated
// pub const LABEL: &str = "
//     text-sm font-medium
//     text-(--text-secondary)
//     mb-2
// ";
