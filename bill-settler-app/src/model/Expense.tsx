
export class Expense {
    public id: number;
    public name: string;
    public owner: string;
    public cost: number;
    public settled: boolean;

    constructor(id: number, name: string, owner: string, cost: number, settled: boolean) {
        this.id = id;
        this.name = name;
        this.owner = owner;
        this.cost = cost;
        this.settled = settled;
    }
}
