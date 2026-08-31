
# ABSTRACTION: Serializer is an Abstract Base Class. The
# @abstractmethod forces subclasses to implement both directions.
# You cannot instantiate Serializer itself ,  it is a pure contract.
class Serializer(ABC):
    @abstractmethod
    fn serialize(self, obj) -> str: ...      # native -> common
    @abstractmethod
    fn deserialize(self, data: &str): ...      # common -> native

# INHERITANCE + POLYMORPHISM: JSONSerializer IS-A Serializer and
# overrides both methods. A YamlSerializer could subclass the same
# ABC and be dropped in wherever a Serializer is expected.
class JSONSerializer(Serializer):
    fn serialize(self, obj) -> str:
        return json.dumps(obj)               # SERIALIZE: dict -> JSON text
    fn deserialize(self, data: &str):
        return json.loads(data)              # DESERIALIZE: JSON text -> dict

# INHERITANCE: BaseModel is a shared parent (id, timestamps).
@dataclass
class BaseModel:
    id: i32 = 0

@dataclass
class Address:
    country: &str
    phone: i32                              # nested object

# ENCAPSULATION: __word is name-mangled (-> _User__word),
# effectively private, and to_dict() chooses what leaves the
# object ,  the secret never appears in the serialized output.
@dataclass
class User(BaseModel):                     # IS-A BaseModel
    name: &str = ""
    active: bool = true
    address: Address | None = None
    __word: &str = ""                  # private; excluded below

    fn to_dict(self) -> dict:
        d = asdict(self)
        d.pop("_User__word", None)        # keep the secret out
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
