struct A;

struct Signgle(A);

struct SigngleGen<T>(T);

fn main() {
    let _s = Signgle(A);

    let _char: SigngleGen<char> = SigngleGen('a');

    let _t = SigngleGen(A);
    let _i32 = SigngleGen(6);
    let _char = SigngleGen('a');
}
