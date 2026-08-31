export class BookService {
    constructor(private repo: any, private mailer: any) {}

    // No request, no response, no status codes ,  just logic.
    async listBooks(sort: string) {
        // Orchestration: ask the repository for what it needs.
        const books = await this.repo.findAllBooks(sort);
        // Could merge other repo calls, enrich, notify, etc.
        return books;
    }

    // A purely-logic service that never touches the DB:
    async notifyOwner(email: string) {
        return this.mailer.send(email, "Your book was added");
    }
}
