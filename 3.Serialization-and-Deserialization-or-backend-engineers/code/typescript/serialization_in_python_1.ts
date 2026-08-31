
# ABSTRACTION: Serializer is an Abstract Base Class. The
# @abstractmethod forces subclasses to implement both directions.
# You cannot instantiate Serializer itthis ,  it is a pure contract.
class Serializer(ABC):
    @abstractmethod
    function serialize(this, obj) -> str: ...      # native -> common
    @abstractmethod
    function deserialize(this, data: string): ...      # common -> native

# INHERITANCE + POLYMORPHISM: JSONSerializer IS-A Serializer and
# overrides both methods. A YamlSerializer could subclass the same
# ABC and be dropped in wherever a Serializer is expected.
class JSONSerializer(Serializer):
    function serialize(this, obj) -> str:
        return json.dumps(obj)               # SERIALIZE: dict -> JSON text
    function deserialize(this, data: string):
        return json.loads(data)              # DESERIALIZE: JSON text -> dict

# INHERITANCE: BaseModel is a shared parent (id, timestamps).
@dataclass
class BaseModel:
    id: number = 0

@dataclass
class Address:
    country: string
    phone: number                              # nested object

# ENCAPSULATION: __word is name-mangled (-> _User__word),
# effectively private, and to_dict() chooses what leaves the
# object ,  the secret never appears in the serialized output.
@dataclass
class User(BaseModel):                     # IS-A BaseModel
    name: string = ""
    active: boolean = true
    address: Address | null = null
    __word: string = ""                  # private; excluded below

    function to_dict(this) -> dict:
        d = asdict(this)
        d.pop("_User__word", null)        # keep the secret out
        return d

if __name__ == "__main__":
    codec: Serializer = JSONSerializer()    # program to the contract

    user = User(id=1, name="Ada",
                address=Address("India", 123456))

    # SERIALIZE ,  native object into the common JSON format
    payload = codec.serialize(user.to_dict())
    print(payload)
    # {"id": 1, "name": "Ada", "active": true,
    #  "address": {"country": "India", "phone": 123456}}

    # DESERIALIZE ,  JSON received over HTTP back into native data
    incoming = '{"name": "Lin", "address": {"country": "IN", "phone": 42}}'
    data = codec.deserialize(incoming)
    print(data["name"], data["address"]["country"])  # Lin IN
