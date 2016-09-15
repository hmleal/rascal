#[cfg(test)]
mod blocks {
    extern crate rascal;

    #[test]
    fn it_eval_retrieve_return_last_statement() {
        let text =
        "begin
           mut x = 10;
           mut y = x + 5;
           return y + 5
         end";

        let result = rascal::eval(String::from(text));
        assert_eq!("20", result);
    }

    #[test]
    fn it_eval_bolean_expressions() {
        let text =
        "begin
           imut x = 1;
           imut y = 2;
           return y == x
         end";
        let result = rascal::eval(String::from(text));
        assert_eq!("false", result);
    }

    #[test]
    fn it_eval_while_blocks() {
        let text =
        "begin
           mut y = 0;
           while y < 4 begin
             y = y + 1
           end;
           return y == 4
        end";
        let result = rascal::eval(String::from(text));
        assert_eq!("true", result);
    }

    #[test]
    fn it_eval_if_blocks() {
        let text =
        "begin
           mut y = 0;
           if y < 4 begin
             y = 4
           end;
           return y == 4
         end";
        let result = rascal::eval(String::from(text));
        assert_eq!("true", result);
    }

    #[test]
    fn it_eval_if_else_blocks() {
        let text =
        "begin
           mut y = 0;
           if y > 4 begin
             y = 4
           else
             y = 10
           end;
           return y == 10
         end";
        let result = rascal::eval(String::from(text));
        assert_eq!("true", result);
    }

    #[test]
    #[should_panic]
    fn it_validate_immutable() {
        let text =
        "begin
           imut y = 0;
           y = 1;
           return y
         end";
        let result = rascal::eval(String::from(text));
    }

    #[test]
    fn it_accepts_mutable() {
        let text =
        "begin
           mut y = 0;
           y = 1;
           return y
        end";
        let result = rascal::eval(String::from(text));
        assert_eq!("1", result);
    }

    #[test]
    #[should_panic]
    fn it_validate_not_declared_var() {
        let text =
        "begin
           imut y = 0;
           x = 1;
           return x
         end";
        let result = rascal::eval(String::from(text));
    }
}