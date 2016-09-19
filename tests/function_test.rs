#[cfg(test)]
mod functions {
    extern crate rascal;

    #[test]
    fn it_eval_function_without_params() {
        let source =
        "begin
           fun add = [] { return 2 + 2 };
           add()
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("4", result);
    }

    #[test]
    fn it_eval_function_with_params() {
        let source =
        "begin
           fun add = [x] { return x + 2 };
           add(2)
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("4", result);
    }

    #[test]
    fn it_eval_function_with_multiple_params() {
        let source =
        "begin
           fun add = [x,y,z] { return x + y + z + 1 };
           add(2,2,2)
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("7", result);
    }

    #[test]
    fn it_eval_complex_function() {
        let source =
        "begin
           mut y = 0;

           fun foo = [x] {
             if x < 10 {
                y = x + 1;
             };

             return y
           };

           foo(6)
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("7", result);
    }

    #[test]
    fn it_eval_recursive_functions() {
        let source =
        "begin
           fun foo = [x] {
             if x < 10 {
                x = x + 1;
                foo(x)
             };

             return x
           };

           foo(6)
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("10", result);
    }

    #[test]
    fn it_eval_expressions_as_args() {
        let source =
        "begin
           fun foo = [x, y] {
             return x + y
           };

           foo((8 + 1), (1 + 1))
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("11", result);
    }

}