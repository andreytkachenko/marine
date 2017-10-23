use service::Service::*;
use nickel::{ Nickel, HttpRouter };
use person::{ PersonController };
use locator::Locator;

macro_rules! routes {
	($loc: ident, $server: ident, $($method:ident $path:tt $dep:ident $type:ident :: $action:ident),*)  => {
		$(
			$server.$method($path, middleware! {|request, response|
				$loc.get::<$type>($dep).$action(request, &response)
		    });
		)*
	}
}

pub fn build_routes(loc: &'static Locator, server: &mut Nickel) { 
	routes! {
		loc, server, 
		
// PERSON CONTROLLER
		get    "/person"     PERSON_CONTROLLER PersonController :: get_persons,
		get    "/person/:id" PERSON_CONTROLLER PersonController :: get_person,
		post   "/person"     PERSON_CONTROLLER PersonController :: create_person,
		delete "/person/:id" PERSON_CONTROLLER PersonController :: delete_person
		
// COMMENTS CONTROLLER
	//	get    "/person/:id/comment" 	  COMMENT_CONTROLLER :: get_comments,
	//	get    "/person/:id/comment/:cid" COMMENT_CONTROLLER :: get_comment(id, cid),
	//	post   "/person/:id/comment"      COMMENT_CONTROLLER :: create_comment(req.get_post_data()),
	//	delete "/person/:id/commnet/:cid" COMMENT_CONTROLLER :: delete_comment(id, cid)
	}
}