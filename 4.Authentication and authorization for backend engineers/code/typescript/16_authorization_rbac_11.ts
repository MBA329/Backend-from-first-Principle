export interface Subject {
    id: string;
    dept: string;
    role: string;
}

export interface Resource {
    ownerID: string;
    dept: string;
    archived: boolean;
}

// Policy: can `s` perform `action` on `r`, given the environment?
export function canEdit(s: Subject, r: Resource, hour: number): boolean {
    if (r.archived) {
        return false;
    }
    const sameDept = s.dept === r.dept;
    const owns = s.id === r.ownerID;
    const businessHours = hour >= 9 && hour < 18;
    // owner OR same-department editor, only in business hours
    return (owns || (sameDept && s.role === 'editor')) && businessHours;
}

// usage inside a handler, after auth has populated the subject:
// if (!canEdit(subj, doc, new Date().getHours())) {
//     res.status(403).send('forbidden');
// }
