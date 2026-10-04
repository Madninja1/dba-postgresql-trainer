use dba_trainer_domain::{QuestionLimit, QuizScope, Topic};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Topics,
    QuizSetup,
    Quiz,
    Statistics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Confirm,
    Back,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeItem {
    Topics,
    GeneralQuiz,
    Statistics,
    Quit,
}

impl HomeItem {
    pub fn label(self) -> &'static str {
        match self {
            Self::Topics => "Тест по теме",
            Self::GeneralQuiz => "Общий тест",
            Self::Statistics => "Статистика",
            Self::Quit => "Выход",
        }
    }
}

pub const HOME_ITEMS: [HomeItem; 4] = [
    HomeItem::Topics,
    HomeItem::GeneralQuiz,
    HomeItem::Statistics,
    HomeItem::Quit,
];

pub const QUESTION_LIMITS: [QuestionLimit; 3] = [
    QuestionLimit::Twenty,
    QuestionLimit::Fifty,
    QuestionLimit::All,
];

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,

    pub home_selected: usize,
    pub topic_selected: usize,
    pub limit_selected: usize,

    pub topics: Vec<Topic>,
    pub quiz_scope: QuizScope,
}

impl App {
    pub fn new(topics: Vec<Topic>) -> Self {
        Self {
            screen: Screen::Home,
            should_quit: false,

            home_selected: 0,
            topic_selected: 0,
            limit_selected: 0,

            topics,
            quiz_scope: QuizScope::AllTopics,
        }
    }

    pub fn selected_limit(&self) -> QuestionLimit {
        QUESTION_LIMITS[self.limit_selected]
    }

    pub fn handle_action(&mut self, action: Action) {
        if action == Action::Quit {
            self.should_quit = true;
            return;
        }

        match self.screen {
            Screen::Home => {
                self.handle_home_action(action);
            }

            Screen::Topics => self.handle_topics_action(action),

            Screen::QuizSetup => self.handle_quiz_setup_action(action),

            Screen::Quiz | Screen::Statistics => {
                if action == Action::Back {
                    self.screen = Screen::Home;
                }
            }
        }
    }

    fn handle_home_action(&mut self, action: Action) {
        match action {
            Action::Up => {
                self.home_selected = previous_index(self.home_selected, HOME_ITEMS.len());
            }

            Action::Down => {
                self.home_selected = next_index(self.home_selected, HOME_ITEMS.len());
            }

            Action::Confirm => match HOME_ITEMS[self.home_selected] {
                HomeItem::Topics => self.screen = Screen::Topics,

                HomeItem::GeneralQuiz => {
                    self.quiz_scope = QuizScope::AllTopics;

                    self.limit_selected = 0;

                    self.screen = Screen::QuizSetup;
                }

                HomeItem::Statistics => {
                    self.screen = Screen::Statistics;
                }

                HomeItem::Quit => {
                    self.should_quit = true;
                }
            },

            Action::Back | Action::Quit => {}
        }
    }

    fn handle_topics_action(&mut self, action: Action) {
        match action {
            Action::Up => {
                self.topic_selected = previous_index(self.topic_selected, self.topics.len());
            }

            Action::Down => {
                self.topic_selected = next_index(self.topic_selected, self.topics.len());
            }

            Action::Confirm => {
                let topic_id = self.topics.get(self.topic_selected).map(|topic| topic.id);

                if let Some(topic_id) = topic_id {
                    self.quiz_scope = QuizScope::Topic(topic_id);

                    self.limit_selected = 0;
                    self.screen = Screen::QuizSetup;
                }
            }

            Action::Back => {
                self.screen = Screen::Home;
            }

            Action::Quit => {}
        }
    }

    fn handle_quiz_setup_action(&mut self, action: Action) {
        match action {
            Action::Up => {
                self.limit_selected = previous_index(self.limit_selected, QUESTION_LIMITS.len());
            }

            Action::Down => {
                self.limit_selected = next_index(self.limit_selected, QUESTION_LIMITS.len());
            }

            Action::Confirm => self.screen = Screen::Quiz,

            Action::Back => {
                self.screen = match self.quiz_scope {
                    QuizScope::Topic(_) => Screen::Topics,

                    QuizScope::AllTopics => Screen::Home,
                }
            }

            Action::Quit => {}
        }
    }
}

fn next_index(current: usize, length: usize) -> usize {
    if length == 0 {
        return 0;
    }

    (current + 1) % length
}

fn previous_index(current: usize, length: usize) -> usize {
    if length == 0 {
        return 0;
    }

    if current == 0 {
        length - 1
    } else {
        current - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_selection_wraps_forward() {
        let mut app = App::new(Vec::new());

        app.home_selected = HOME_ITEMS.len() - 1;

        app.handle_action(Action::Down);

        assert_eq!(app.home_selected, 0);
    }

    #[test]
    fn home_selection_wraps_backward() {
        let mut app = App::new(Vec::new());

        app.handle_action(Action::Up);

        assert_eq!(app.home_selected, HOME_ITEMS.len() - 1);
    }

    #[test]
    fn general_quiz_opens_quiz_setup() {
        let mut app = App::new(Vec::new());

        app.home_selected = 1;

        app.handle_action(Action::Confirm);

        assert_eq!(app.screen, Screen::QuizSetup);
        assert_eq!(app.quiz_scope, QuizScope::AllTopics);
    }

    #[test]
    fn selecting_topic_opens_quiz_setup() {
        let topics = vec![Topic {
            id: dba_trainer_domain::TopicId(42),
            slug: String::from("architecture"),
            title: String::from("Архитектура PostgreSQL"),
            description: None,
        }];

        let mut app = App::new(topics);

        app.screen = Screen::Topics;

        app.handle_action(Action::Confirm);

        assert_eq!(
            app.quiz_scope,
            QuizScope::Topic(dba_trainer_domain::TopicId(42))
        );

        assert_eq!(app.screen, Screen::QuizSetup);
    }
}
