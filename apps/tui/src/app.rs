use dba_trainer_application::{RepositoryError, TrainerService};

use dba_trainer_domain::{
    AnswerOptionId, AnswerResult, Question, QuestionId, QuestionLimit, QuestionType, QuizScope,
    SessionConfig, SessionId, SessionProgress, Topic, TrainingStats,
};

use dba_trainer_storage_sqlite::SqliteRepository;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,

    ResumeSession,

    Topics,
    QuizSetup,

    Quiz,
    Feedback,

    CancelSession,

    Results,
    Statistics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Toggle,
    Confirm,
    Back,
    Quit,
    ClearStatistics,
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

    pub option_selected: usize,

    pub topics: Vec<Topic>,

    pub quiz_scope: QuizScope,

    pub current_question: Option<Question>,

    pub selected_answer_ids: Vec<AnswerOptionId>,

    pub feedback: Option<AnswerResult>,

    pub answered_questions: usize,

    pub correct_answers: usize,

    pub total_questions: usize,

    pub error_message: Option<String>,

    pub resume_session: Option<SessionProgress>,

    pub decision_selected: usize,

    pub statistics: Option<TrainingStats>,

    cancel_return_screen: Screen,

    session_id: Option<SessionId>,

    service: TrainerService<SqliteRepository>,
}

impl App {
    pub fn new(service: TrainerService<SqliteRepository>) -> Result<Self, RepositoryError> {
        let topics = service.topics()?;

        let resume_session = service.active_session()?;

        let screen = if resume_session.is_some() {
            Screen::ResumeSession
        } else {
            Screen::Home
        };

        Ok(Self {
            screen,

            should_quit: false,

            home_selected: 0,

            topic_selected: 0,

            limit_selected: 0,

            option_selected: 0,

            topics,

            quiz_scope: QuizScope::AllTopics,

            current_question: None,

            selected_answer_ids: Vec::new(),

            feedback: None,

            answered_questions: 0,

            correct_answers: 0,

            total_questions: 0,

            error_message: None,

            session_id: None,

            resume_session,

            decision_selected: 0,

            statistics: None,

            cancel_return_screen: Screen::Quiz,

            service,
        })
    }

    pub fn selected_limit(&self) -> QuestionLimit {
        QUESTION_LIMITS[self.limit_selected]
    }

    pub fn handle_action(&mut self, action: Action) {
        if action == Action::Quit {
            self.should_quit = true;

            return;
        }

        self.error_message = None;

        let result = match self.screen {
            Screen::Home => self.handle_home_action(action),

            Screen::Topics => self.handle_topics_action(action),

            Screen::QuizSetup => self.handle_quiz_setup_action(action),

            Screen::Quiz => self.handle_quiz_action(action),

            Screen::Feedback => self.handle_feedback_action(action),

            Screen::Results => self.handle_results_action(action),

            Screen::ResumeSession => self.handle_resume_action(action),

            Screen::CancelSession => self.handle_cancel_action(action),

            Screen::Statistics => self.handle_statistics_action(action),
        };

        if let Err(error) = result {
            self.error_message = Some(error.to_string());
        }
    }

    fn handle_home_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        match action {
            Action::Up => {
                self.home_selected = previous_index(self.home_selected, HOME_ITEMS.len());
            }

            Action::Down => {
                self.home_selected = next_index(self.home_selected, HOME_ITEMS.len());
            }

            Action::Confirm => match HOME_ITEMS[self.home_selected] {
                HomeItem::Topics => {
                    self.screen = Screen::Topics;
                }

                HomeItem::GeneralQuiz => {
                    self.quiz_scope = QuizScope::AllTopics;

                    self.limit_selected = 0;

                    self.screen = Screen::QuizSetup;
                }

                HomeItem::Statistics => {
                    self.statistics = Some(self.service.statistics()?);

                    self.screen = Screen::Statistics;
                }

                HomeItem::Quit => {
                    self.should_quit = true;
                }
            },

            Action::Back | Action::Toggle | Action::Quit | Action::ClearStatistics => {}
        }

        Ok(())
    }

    fn handle_topics_action(&mut self, action: Action) -> Result<(), RepositoryError> {
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

            Action::Toggle | Action::Quit | Action::ClearStatistics => {}
        }

        Ok(())
    }

    fn handle_quiz_setup_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        match action {
            Action::Up => {
                self.limit_selected = previous_index(self.limit_selected, QUESTION_LIMITS.len());
            }

            Action::Down => {
                self.limit_selected = next_index(self.limit_selected, QUESTION_LIMITS.len());
            }

            Action::Confirm => {
                self.start_session()?;
            }

            Action::Back => {
                self.screen = match self.quiz_scope {
                    QuizScope::Topic(_) => Screen::Topics,

                    QuizScope::AllTopics => Screen::Home,
                };
            }

            Action::Toggle | Action::Quit | Action::ClearStatistics => {}
        }

        Ok(())
    }

    fn handle_quiz_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        let option_count = self
            .current_question
            .as_ref()
            .map(|question| question.options.len())
            .unwrap_or(0);

        match action {
            Action::Up => {
                self.option_selected = previous_index(self.option_selected, option_count);
            }

            Action::Down => {
                self.option_selected = next_index(self.option_selected, option_count);
            }

            Action::Toggle => {
                self.toggle_current_option();
            }

            Action::Confirm => {
                self.submit_current_answer()?;
            }

            Action::Back => {
                self.cancel_return_screen = Screen::Quiz;

                self.decision_selected = 0;

                self.screen = Screen::CancelSession;
            }

            Action::Quit | Action::ClearStatistics => {}
        }

        Ok(())
    }

    fn handle_feedback_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        match action {
            Action::Confirm => {
                self.load_current_question()?;
            }

            Action::Back => {
                self.cancel_return_screen = Screen::Feedback;

                self.decision_selected = 0;

                self.screen = Screen::CancelSession;
            }

            _ => {}
        }

        Ok(())
    }

    fn handle_results_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        if matches!(action, Action::Confirm | Action::Back) {
            self.reset_quiz();

            self.screen = Screen::Home;
        }

        Ok(())
    }

    fn handle_resume_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        match action {
            Action::Up | Action::Down => {
                self.decision_selected = if self.decision_selected == 0 { 1 } else { 0 };
            }

            Action::Confirm => match self.decision_selected {
                0 => {
                    self.resume_quiz()?;
                }

                _ => {
                    self.cancel_saved_quiz()?;
                }
            },

            Action::Back | Action::Toggle | Action::Quit | Action::ClearStatistics => {}
        }

        Ok(())
    }

    fn handle_cancel_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        match action {
            Action::Up | Action::Down => {
                self.decision_selected = if self.decision_selected == 0 { 1 } else { 0 };
            }

            Action::Confirm => {
                if self.decision_selected == 0 {
                    self.screen = self.cancel_return_screen;
                } else {
                    let session_id = self.session_id.ok_or_else(|| {
                        RepositoryError::InvalidState(String::from("active session is missing"))
                    })?;

                    self.service.cancel_session(session_id)?;

                    self.reset_quiz();

                    self.screen = Screen::Home;
                }
            }

            Action::Back => {
                self.screen = self.cancel_return_screen;
            }

            Action::Toggle | Action::Quit | Action::ClearStatistics => {}
        }

        Ok(())
    }

    fn handle_statistics_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        match action {
            Action::ClearStatistics => {
                self.service.clear_statistics()?;

                self.statistics = Some(self.service.statistics()?);
            }

            Action::Back | Action::Confirm => {
                self.screen = Screen::Home;
            }

            _ => {}
        }

        Ok(())
    }

    fn cancel_saved_quiz(&mut self) -> Result<(), RepositoryError> {
        let session_id = self
            .resume_session
            .as_ref()
            .map(|session| session.id)
            .ok_or_else(|| {
                RepositoryError::InvalidState(String::from("resume session is missing"))
            })?;

        self.service.cancel_session(session_id)?;

        self.resume_session = None;

        self.reset_quiz();

        self.screen = Screen::Home;

        Ok(())
    }

    fn resume_quiz(&mut self) -> Result<(), RepositoryError> {
        let progress = self.resume_session.clone().ok_or_else(|| {
            RepositoryError::InvalidState(String::from("resume session is missing"))
        })?;

        self.session_id = Some(progress.id);

        self.quiz_scope = progress.scope;

        self.total_questions = progress.total_questions;

        self.answered_questions = progress.answered_questions;

        self.correct_answers = progress.correct_answers;

        self.feedback = None;

        self.resume_session = None;

        self.load_current_question()
    }

    fn start_session(&mut self) -> Result<(), RepositoryError> {
        let config = SessionConfig {
            scope: self.quiz_scope,

            limit: self.selected_limit(),
        };

        let session = self.service.start_session(&config)?;

        self.session_id = Some(session.id);

        self.total_questions = session.total_questions();

        self.answered_questions = 0;

        self.correct_answers = 0;

        self.feedback = None;

        self.load_current_question()
    }

    fn load_current_question(&mut self) -> Result<(), RepositoryError> {
        let session_id = self.session_id.ok_or_else(|| {
            RepositoryError::InvalidState(String::from("quiz session is missing"))
        })?;

        let question = self.service.current_question(session_id)?;

        match question {
            Some(question) => {
                self.current_question = Some(question);

                self.option_selected = 0;

                self.selected_answer_ids.clear();

                self.feedback = None;

                self.screen = Screen::Quiz;
            }

            None => {
                self.current_question = None;

                self.selected_answer_ids.clear();

                self.feedback = None;

                self.screen = Screen::Results;
            }
        }

        Ok(())
    }

    fn toggle_current_option(&mut self) {
        let option_id = self
            .current_question
            .as_ref()
            .filter(|question| question.question_type == QuestionType::MultipleChoice)
            .and_then(|question| question.options.get(self.option_selected))
            .map(|option| option.id);

        let Some(option_id) = option_id else {
            return;
        };

        if let Some(position) = self
            .selected_answer_ids
            .iter()
            .position(|id| *id == option_id)
        {
            self.selected_answer_ids.remove(position);
        } else {
            self.selected_answer_ids.push(option_id);
        }
    }

    fn submit_current_answer(&mut self) -> Result<(), RepositoryError> {
        let question = self.current_question.as_ref().ok_or_else(|| {
            RepositoryError::InvalidState(String::from("current question is missing"))
        })?;

        let question_id = question.id;

        let selected_ids = match question.question_type {
            QuestionType::SingleChoice => {
                let option_id = question
                    .options
                    .get(self.option_selected)
                    .map(|option| option.id)
                    .ok_or_else(|| {
                        RepositoryError::InvalidState(String::from("selected option is missing"))
                    })?;

                vec![option_id]
            }

            QuestionType::MultipleChoice => {
                if self.selected_answer_ids.is_empty() {
                    self.error_message =
                        Some(String::from("Выберите хотя бы один вариант ответа."));

                    return Ok(());
                }

                self.selected_answer_ids.clone()
            }
        };

        self.submit_answers(question_id, selected_ids)
    }

    fn submit_answers(
        &mut self,
        question_id: QuestionId,

        selected_ids: Vec<AnswerOptionId>,
    ) -> Result<(), RepositoryError> {
        let session_id = self.session_id.ok_or_else(|| {
            RepositoryError::InvalidState(String::from("quiz session is missing"))
        })?;

        let result = self
            .service
            .submit_answer(session_id, question_id, &selected_ids)?;

        self.answered_questions += 1;

        if result.is_correct {
            self.correct_answers += 1;
        }

        self.feedback = Some(result);

        self.screen = Screen::Feedback;

        Ok(())
    }

    fn reset_quiz(&mut self) {
        self.session_id = None;

        self.current_question = None;

        self.feedback = None;

        self.selected_answer_ids.clear();

        self.option_selected = 0;

        self.answered_questions = 0;

        self.correct_answers = 0;

        self.total_questions = 0;

        self.error_message = None;
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
