
pub type Key = &'static str;

#[derive(Debug, Clone, Copy)]
pub struct Screenshot {
    pub src: &'static str,
    pub alt: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct Metric {
    pub value: &'static str,
    pub label: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectLink {
    pub label: Key,
    pub url: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct VideoAsset {
    pub mp4_src: &'static str,
    pub webm_src: Option<&'static str>,
    pub poster_src: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Chrome {
    pub skip_to_content: Key,
    pub nav_act1: Key,
    pub nav_act2: Key,
    pub nav_act3: Key,
    pub nav_credits: Key,
    pub lang_es: Key,
    pub lang_en: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct RouteMeta {
    pub title: Key,
    pub description: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct Opening {
    pub name: Key,
    pub role: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct Intertitle {
    pub label: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct IntertitleTwo {
    pub label: Key,
    pub line1: Key,
    pub line2: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct ActOne {
    pub beat1: Key,
    pub verbs: [Key; 4],
    pub layers: Key,
    pub stack: Key,
    pub prose: Key,
    pub areas_montage: Option<Key>,
    pub screenshots: Option<&'static [Screenshot]>,
    pub metrics: Option<&'static [Metric]>,
    pub project_links: Option<&'static [ProjectLink]>,
}

#[derive(Debug, Clone, Copy)]
pub struct ActTwo {
    pub heading: Key,
    pub subheading: Key,
    pub facts: Key,
    // Split into literal pieces so the source never holds the `app!(` substring, which telar-project refuses in a hand-written file.
    pub code_snippet: &'static str,
    pub note: Key,
    pub screenshots: Option<&'static [Screenshot]>,
    pub project_links: Option<&'static [ProjectLink]>,
    pub video: Option<VideoAsset>,
}

#[derive(Debug, Clone, Copy)]
pub struct ActThree {
    pub title: Key,
    pub bio_line1: Key,
    pub bio_line2: Key,
    pub subtitle: Key,
    pub problem: Key,
    pub video: Option<VideoAsset>,
}

#[derive(Debug, Clone, Copy)]
pub struct Education {
    pub name: Key,
    pub period: Key,
    pub program: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct Credits {
    pub about_heading: Key,
    pub bio_line1: Key,
    pub bio_line2: Key,
    pub experience_summary: Key,
    pub education: [Education; 2],
    pub stack_heading: Key,
    pub stack: Key,
    pub contact_heading: Key,
    pub contact_email_label: Key,
    pub contact_github_label: Key,
    pub contact_linkedin_label: Key,
    pub contact_cv_label: Key,
    pub contact_email: &'static str,
    pub contact_github_url: &'static str,
    pub contact_linkedin_url: &'static str,
    pub contact_cv_url: Option<&'static str>,
    pub made_with: Key,
    pub end_card: Key,
}

#[derive(Debug, Clone, Copy)]
pub struct Content {
    pub route_home: RouteMeta,
    pub chrome: Chrome,
    pub opening: Opening,
    pub intertitle1: Intertitle,
    pub act1: ActOne,
    pub intertitle2: IntertitleTwo,
    pub act2: ActTwo,
    pub act3: ActThree,
    pub credits: Credits,
}

impl Content {
    pub fn referenced_keys(&self) -> Vec<Key> {
        let mut keys = vec![
            self.route_home.title,
            self.route_home.description,
            self.chrome.skip_to_content,
            self.chrome.nav_act1,
            self.chrome.nav_act2,
            self.chrome.nav_act3,
            self.chrome.nav_credits,
            self.chrome.lang_es,
            self.chrome.lang_en,
            self.opening.name,
            self.opening.role,
            self.intertitle1.label,
            self.act1.beat1,
            self.act1.layers,
            self.act1.stack,
            self.act1.prose,
            self.intertitle2.label,
            self.intertitle2.line1,
            self.intertitle2.line2,
            self.act2.heading,
            self.act2.subheading,
            self.act2.facts,
            self.act2.note,
            self.act3.title,
            self.act3.bio_line1,
            self.act3.bio_line2,
            self.act3.subtitle,
            self.act3.problem,
            self.credits.about_heading,
            self.credits.bio_line1,
            self.credits.bio_line2,
            self.credits.experience_summary,
            self.credits.stack_heading,
            self.credits.stack,
            self.credits.contact_heading,
            self.credits.contact_email_label,
            self.credits.contact_github_label,
            self.credits.contact_linkedin_label,
            self.credits.contact_cv_label,
            self.credits.made_with,
            self.credits.end_card,
        ];
        keys.extend(self.act1.verbs);
        keys.extend(self.act1.areas_montage);
        for screenshot in self.act1.screenshots.into_iter().flatten() {
            keys.push(screenshot.alt);
        }
        for metric in self.act1.metrics.into_iter().flatten() {
            keys.push(metric.label);
        }
        for link in self.act1.project_links.into_iter().flatten() {
            keys.push(link.label);
        }
        for screenshot in self.act2.screenshots.into_iter().flatten() {
            keys.push(screenshot.alt);
        }
        for link in self.act2.project_links.into_iter().flatten() {
            keys.push(link.label);
        }
        for education in self.credits.education {
            keys.push(education.name);
            keys.push(education.period);
            keys.push(education.program);
        }
        keys
    }
}

pub const TELAR_REPO_URL: &str = "https://github.com/AdrielGBM/telar";

pub const CONTENT: Content = Content {
    route_home: RouteMeta {
        title: "route.home.title",
        description: "route.home.description",
    },
    chrome: Chrome {
        skip_to_content: "chrome.skip_to_content",
        nav_act1: "chrome.nav_act1",
        nav_act2: "chrome.nav_act2",
        nav_act3: "chrome.nav_act3",
        nav_credits: "chrome.nav_credits",
        lang_es: "chrome.lang_es",
        lang_en: "chrome.lang_en",
    },
    opening: Opening {
        name: "opening.name",
        role: "opening.role",
    },
    intertitle1: Intertitle {
        label: "intertitle1.label",
    },
    act1: ActOne {
        beat1: "act1.beat1",
        verbs: [
            "act1.verb_deploy",
            "act1.verb_migrate",
            "act1.verb_scale",
            "act1.verb_maintain",
        ],
        layers: "act1.layers",
        stack: "act1.stack",
        prose: "act1.prose",
        // Stays empty until the employer approves naming the business areas.
        areas_montage: None,
        screenshots: None,
        metrics: None,
        project_links: None,
    },
    intertitle2: IntertitleTwo {
        label: "intertitle2.label",
        line1: "intertitle2.line1",
        line2: "intertitle2.line2",
    },
    act2: ActTwo {
        heading: "act2.heading",
        subheading: "act2.subheading",
        facts: "act2.facts",
        code_snippet: "[view]\nbox fill:$theme.surface stroke:$theme.border radius:16 pad:24 gap:10 axis:col\n    text \"{props.title}\" font_size:18 color:$theme.dark\n    text \"{props.body}\" font_size:14 color:$theme.muted",
        note: "act2.note",
        screenshots: None,
        project_links: Some(&[ProjectLink {
            label: "act2.link_repo",
            url: TELAR_REPO_URL,
        }]),
        video: None,
    },
    act3: ActThree {
        title: "act3.title",
        bio_line1: "act3.bio_line1",
        bio_line2: "act3.bio_line2",
        subtitle: "act3.subtitle",
        problem: "act3.problem",
        video: None,
    },
    credits: Credits {
        about_heading: "credits.about_heading",
        bio_line1: "credits.bio_line1",
        bio_line2: "credits.bio_line2",
        experience_summary: "credits.experience_summary",
        education: [
            Education {
                name: "credits.education_mayor_name",
                period: "credits.education_mayor_period",
                program: "credits.education_mayor_program",
            },
            Education {
                name: "credits.education_devf_name",
                period: "credits.education_devf_period",
                program: "credits.education_devf_program",
            },
        ],
        stack_heading: "credits.stack_heading",
        stack: "credits.stack",
        contact_heading: "credits.contact_heading",
        contact_email_label: "credits.contact_email_label",
        contact_github_label: "credits.contact_github_label",
        contact_linkedin_label: "credits.contact_linkedin_label",
        contact_cv_label: "credits.contact_cv_label",
        contact_email: "adrielgbm@gmail.com",
        contact_github_url: "https://github.com/AdrielGBM",
        contact_linkedin_url: "https://www.linkedin.com/in/AdrielGBM",
        contact_cv_url: None,
        made_with: "credits.made_with",
        end_card: "credits.end_card",
    },
};

#[cfg(test)]
#[path = "content_test.rs"]
mod tests;
