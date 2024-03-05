pub struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut strack = Vec::new();
        let s: Vec<&str> = path.split("/").collect();
        for i in s {
            match i {
                "" => continue,
                "." => continue,
                ".." => {
                    strack.pop();
                }

                _ => {
                    strack.push(i);
                }
            }
        }
        if strack.len() > 0 {
            let mut result = String::from("");
            for i in strack {
                result.push_str("/");
                result.push_str(i);
            }
            return result;
        }
        String::from("/")
    }
}
