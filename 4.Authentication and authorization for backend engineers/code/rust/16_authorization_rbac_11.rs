pub struct Subject {
    pub id: String,
    pub dept: String,
    pub role: String,
}

pub struct Resource {
    pub owner_id: String,
    pub dept: String,
    pub archived: bool,
}

// Policy: can `s` perform `action` on `r`, given the environment?
pub fn can_edit(s: &Subject, r: &Resource, hour: u32) -> bool {
    if r.archived {
        return false;
    }
    let same_dept = s.dept == r.dept;
    let owns = s.id == r.owner_id;
    let business_hours = hour >= 9 && hour < 18;
    
    // owner OR same-department editor, only in business hours
    (owns || (same_dept && s.role == "editor")) && business_hours
}

// usage inside a handler, after auth has populated the subject:
// use chrono::Local;
// if !can_edit(&subj, &doc, Local::now().hour()) {
//     return HttpResponse::Forbidden().body("forbidden");
// }
