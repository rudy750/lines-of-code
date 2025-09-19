// Sample TypeScript file
interface User {
    name: string;
    age: number;
}

// Main function
function greetUser(user: User): string {
    return `Hello, ${user.name}!`;
}

const user: User = {
    name: "Alice",
    age: 30
};

console.log(greetUser(user));