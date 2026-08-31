/**
 * In TypeScript, Protobuf messages are typically compiled into interfaces or classes.
 * We use a library like ts-proto or grpc-tools to generate these.
 * Below is the conceptual equivalent of the generated TypeScript types
 * for the Protobuf definitions.
 */

// An enum is a fixed set of named values. The first MUST be 0 (the default).
export enum Role {
  ROLE_UNSPECIFIED = 0,         // 0 is the implicit default, reserve it
  ROLE_MEMBER = 1,
  ROLE_ADMIN = 2,
}

// A message is a typed record.
export interface User {
  id: string;             // the field number in proto is used for binary encoding
  email: string;
  fullName: string;
  role: Role;             // a nested enum
  tags: string[];         // "repeated" = a list/array of strings
  createdAt: number;      // unix seconds; ts-proto can map int64 to number or string
}

export interface GetUserRequest {
  id: string;
}

export interface GetUserResponse {
  user: User | undefined; // messages nest inside messages
}

export interface CreateUserRequest {
  email: string;
  fullName: string;
  phone?: string;         // "optional" tracks presence
}

// A SERVICE is a set of methods. Each takes one message and returns one message.
export interface UserService {
  getUser(request: GetUserRequest): Promise<GetUserResponse>;
  createUser(request: CreateUserRequest): Promise<User>;
}
