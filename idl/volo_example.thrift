namespace rs volo.example

enum Commands {
    Get,
    Set,
    Del,
    Ping,
    Subscribe,
    Publish,
}

struct GetItemRequest {
    1: required Commands cmd,
    2: optional list<string> args,
}

struct GetItemResponse {
    1: required bool ok,
    2: optional string msg,
}

service ItemService {
    GetItemResponse GetItem (1: GetItemRequest req),
}

