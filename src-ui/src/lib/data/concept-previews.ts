// C concept descriptions for educational preview (keys match levels.json concept field)
export interface ConceptPreview {
    title: string;
    preview: string;
    skills: string[];
}

export const CONCEPT_PREVIEWS: Record<string, ConceptPreview> = {
    // Level 1: Return Values
    'return values': {
        title: 'Your First C Function',
        preview: 'Learn how functions communicate results back to the caller using return statements. This is the foundation of all C programming.',
        skills: ['return keyword', 'function structure', 'integer results']
    },
    // Level 2: Variables
    'variables': {
        title: 'Data & Computation',
        preview: 'Variables store data, and operators transform it. Master the building blocks of computation in C.',
        skills: ['int variables', 'arithmetic operators', 'expressions']
    },
    // Level 3: Conditionals
    'if/else': {
        title: 'Decision Making',
        preview: 'Programs make decisions using conditional logic. Learn to branch execution based on conditions.',
        skills: ['if statements', 'else clauses', 'comparison operators']
    },
    // Level 4: Loops
    'loops': {
        title: 'Repetition & Iteration',
        preview: 'Loops let you repeat actions efficiently. Essential for processing collections and building algorithms.',
        skills: ['while loops', 'for loops', 'loop control']
    },
    // Level 5: Arrays
    'arrays': {
        title: 'Collections of Data',
        preview: 'Arrays store multiple values of the same type. Learn to index, iterate, and manipulate sequences.',
        skills: ['array declaration', 'indexing', 'iteration']
    },
    // Level 6: Void Functions
    'void functions': {
        title: 'Actions Without Results',
        preview: 'Some functions perform actions without returning values. Learn when and how to use void functions.',
        skills: ['void keyword', 'side effects', 'pointer parameters']
    },
    // Level 7: Complex Expressions
    'complex return expressions': {
        title: 'Combining Operations',
        preview: 'Build sophisticated calculations by combining operators. Master precedence and expression evaluation.',
        skills: ['operator precedence', 'compound expressions', 'multi-step logic']
    },
    // Level 8: Stack Frames
    'stack frames': {
        title: 'Function Execution',
        preview: 'Understand how C manages function calls using the stack. Each call creates its own execution context.',
        skills: ['call stack', 'local variables', 'function scope']
    },
    // Level 9: Scope
    'scope': {
        title: 'Variable Visibility',
        preview: 'Variables exist in specific scopes and have defined lifetimes. Learn the rules governing variable access.',
        skills: ['block scope', 'variable lifetime', 'shadowing']
    },
    // Level 10: Recursion
    'recursion': {
        title: 'Self-Referential Functions',
        preview: 'Functions can call themselves! Learn this powerful technique for solving problems that have recursive structure.',
        skills: ['base cases', 'recursive calls', 'stack depth']
    },
    // Level 11: Pointers - Address-of
    'pointers - address-of': {
        title: 'Memory Addresses',
        preview: 'The & operator reveals where variables live in memory. This is your first step into the world of pointers.',
        skills: ['& operator', 'memory addresses', 'pointer concept']
    },
    // Level 12: Pointers - Declaration
    'pointers - declaration': {
        title: 'Pointer Variables',
        preview: 'Pointer variables store memory addresses. Learn to declare and initialize them correctly.',
        skills: ['pointer syntax', '* in declarations', 'NULL pointers']
    },
    // Level 13: Pointers - Dereference
    'pointers - dereference': {
        title: 'Following Pointers',
        preview: 'The * operator follows a pointer to access the value it points to. Master the duality of pointer operations.',
        skills: ['* operator', 'reading values', 'writing through pointers']
    },
    // Level 14: Pointer Arithmetic
    'pointer arithmetic': {
        title: 'Navigating Memory',
        preview: 'Add and subtract from pointers to navigate through arrays and memory blocks. Pointers know their element size!',
        skills: ['pointer + int', 'array traversal', 'pointer subtraction']
    },
    // Level 15: NULL Safety
    'null safety': {
        title: 'Defensive Programming',
        preview: 'NULL pointers cause crashes if dereferenced. Learn to check for NULL and write safer code.',
        skills: ['NULL checks', 'defensive coding', 'error handling']
    },
    // Level 16: Struct Definition
    'struct definition': {
        title: 'Custom Data Types',
        preview: 'Group related data into custom structures. This is how you create meaningful data types in C.',
        skills: ['struct keyword', 'member fields', 'typedef']
    },
    // Level 17: Struct Members
    'struct members': {
        title: 'Accessing Structure Data',
        preview: 'Use the dot operator to access individual members of a struct. Build and query complex data.',
        skills: ['dot operator', 'member access', 'struct initialization']
    },
    // Level 18: Struct Pointers
    'struct pointers': {
        title: 'The Arrow Operator',
        preview: 'When you have a pointer to a struct, use -> to access members. This is essential for dynamic data structures.',
        skills: ['-> operator', 'struct pointers', 'indirect access']
    },
    // Level 19: Nested Structs
    'nested structs': {
        title: 'Composition',
        preview: 'Structs can contain other structs. Build complex data models through composition.',
        skills: ['nested access', 'data modeling', 'struct composition']
    },
    // Level 20: Arrays of Structs
    'struct arrays': {
        title: 'Collections of Records',
        preview: 'Combine arrays and structs to create databases and collections of complex records.',
        skills: ['struct arrays', 'record iteration', 'data tables']
    },
    // Level 21: malloc Basics
    'malloc basics': {
        title: 'Dynamic Memory',
        preview: 'Request memory at runtime with malloc. Control exactly how much memory your program uses.',
        skills: ['malloc()', 'sizeof', 'heap allocation']
    },
    // Level 22: free
    'free': {
        title: 'Memory Cleanup',
        preview: 'Every malloc needs a matching free. Learn to release memory and prevent resource exhaustion.',
        skills: ['free()', 'memory lifecycle', 'cleanup patterns']
    },
    // Level 23: Memory Leaks
    'memory leaks': {
        title: 'Finding Lost Memory',
        preview: 'Memory leaks accumulate over time. Learn to detect and prevent them for robust programs.',
        skills: ['leak detection', 'ownership tracking', 'cleanup discipline']
    },
    // Level 24: realloc
    'realloc': {
        title: 'Growing & Shrinking Allocations',
        preview: 'Resize existing allocations with realloc. Build dynamic arrays that grow as needed.',
        skills: ['realloc()', 'resize patterns', 'dynamic arrays']
    },
    // Level 25: Linked Lists
    'linked lists': {
        title: 'Dynamic Data Structures',
        preview: 'Build your first data structure! Linked lists use pointers to chain nodes together dynamically.',
        skills: ['node structs', 'list traversal', 'insertion/deletion']
    }
};

// Default fallback for unknown concepts
export function getConceptPreview(concept: string): ConceptPreview {
    return CONCEPT_PREVIEWS[concept] ?? {
        title: concept,
        preview: `Master the concept of ${concept} through hands-on coding challenges.`,
        skills: ['practical application', 'problem solving']
    };
}
