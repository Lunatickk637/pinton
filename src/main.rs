mod interpreter;

fn main() {
    //for token in interpreter::tokenizer::tokenize("x = 10 print 10".to_string()) {
		//println!("{:#?}", token)
	//}
	interpreter::execute("print 200 print 10 print 11".to_string());
}
