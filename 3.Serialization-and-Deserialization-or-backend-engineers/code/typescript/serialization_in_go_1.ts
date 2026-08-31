// ABSTRACTION: Serializer names a capability - "turn data into
// bytes and back" - without binding to a concrete format. Code
// depends on this contract, so JSON could be swapped for another
// format with zero changes to callers.
export interface Serializer {
    serialize(v: any): string;         // native -> common format
    deserialize(data: string): any;    // common format -> native
}

// POLYMORPHISM: JSONSerializer implements Serializer. Any other
// format (YAML, Protobuf) could implement the same interface and
// be used interchangeably through a Serializer variable.
export class JSONSerializer implements Serializer {
    public serialize(v: any): string {
        return JSON.stringify(v); // SERIALIZE: object -> JSON string
    }
    public deserialize(data: string): any {
        return JSON.parse(data);  // DESERIALIZE: JSON string -> object
    }
}

// "INHERITANCE" / COMPOSITION: BaseModel holds shared fields;
// extending it reuses them.
export class BaseModel {
    public id: number = 0;
    public created_at: Date = new Date();
}

export interface Address {
    country: string;
    phone: number;
}

// ENCAPSULATION: fields can be private (or excluded via toJSON).
// We use a custom toJSON method to ensure the password never leaks
// over the wire.
export class User extends BaseModel {
    public name: string;
    public active: boolean;
    public address: Address;
    private password_: string; // hidden from JSON output

    constructor(id: number, name: string, active: boolean, address: Address, password_: string) {
        super();
        this.id = id;
        this.name = name;
        this.active = active;
        this.address = address;
        this.password_ = password_;
    }

    // Control the JSON shape
    public toJSON(): any {
        return {
            id: this.id,
            created_at: this.created_at.toISOString(),
            name: this.name,
            active: this.active,
            address: this.address
        };
    }
}

if (require.main === module) {
    const codec: Serializer = new JSONSerializer(); // program to the interface

    const u = new User(
        1,
        "Ada",
        true,
        { country: "India", phone: 123456 },
        "supersecret"
    );

    // SERIALIZE - native object into the common JSON format
    const out = codec.serialize(u);
    console.log(out);
    // {"id":1,"created_at":"...","name":"Ada","active":true,
    //  "address":{"country":"India","phone":123456}}

    // DESERIALIZE - JSON received over HTTP back into a native object
    const incoming = '{"name":"Lin","address":{"country":"IN","phone":42}}';
    const back = codec.deserialize(incoming);
    console.log(back.name, back.address.country); // Lin IN
}
