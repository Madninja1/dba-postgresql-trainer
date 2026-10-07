use dba_trainer_application::{RepositoryError, TrainerService};

use dba_trainer_domain::{
    AnswerOptionId, AnswerResult, Question, QuestionId, QuestionLimit, QuestionType, QuizScope,
    SessionConfig, SessionId, SessionProgress, StatisticsFilter, StatisticsLimit, StatisticsScope,
    Topic, TopicId, TrainingStats,
};

use dba_trainer_storage_sqlite::SqliteRepository;

use crate::localization::{UiLanguage, UiStrings, ui_strings};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,

    ResumeSession,

    Topics,
    Courses,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatisticsView {
    Root,
    Courses,
    Course(String),
    Filters {
        scope: StatisticsScope,
        title: String,
    },
    Modes,
    Detail {
        filter: StatisticsFilter,
        title: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeItem {
    Topics,
    GeneralQuiz,
    Statistics,
    Language,
    Quit,
}

impl HomeItem {
    pub fn label(self, strings: &UiStrings, language: UiLanguage) -> String {
        match self {
            Self::Topics => strings.topic_quiz.to_string(),
            Self::GeneralQuiz => strings.general_quiz.to_string(),
            Self::Statistics => strings.statistics.to_string(),
            Self::Language => format!("{}: {}", strings.language, strings.language_name(language)),
            Self::Quit => strings.quit.to_string(),
        }
    }
}

pub const HOME_ITEMS: [HomeItem; 5] = [
    HomeItem::Topics,
    HomeItem::GeneralQuiz,
    HomeItem::Statistics,
    HomeItem::Language,
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
    pub language: UiLanguage,

    pub home_selected: usize,
    pub topic_selected: usize,
    pub course_selected: usize,
    pub limit_selected: usize,

    pub option_selected: usize,

    pub topics: Vec<Topic>,

    pub quiz_scope: QuizScope,

    pub current_question: Option<Question>,

    pub selected_answer_ids: Vec<AnswerOptionId>,

    pub feedback: Option<AnswerResult>,

    pub feedback_scroll: u16,

    pub answered_questions: usize,

    pub correct_answers: usize,

    pub total_questions: usize,

    pub error_message: Option<String>,

    pub resume_session: Option<SessionProgress>,

    pub decision_selected: usize,

    pub statistics: Option<TrainingStats>,

    pub statistics_view: StatisticsView,

    pub statistics_selected: usize,

    statistics_course_summaries: Vec<(String, TrainingStats)>,

    statistics_topic_summaries: Vec<(TopicId, TrainingStats)>,

    statistics_mode_summaries: Vec<(StatisticsLimit, TrainingStats)>,

    statistics_filter_summaries: Vec<(StatisticsLimit, TrainingStats)>,

    statistics_history: Vec<(StatisticsView, usize)>,

    cancel_return_screen: Screen,

    session_id: Option<SessionId>,

    service: TrainerService<SqliteRepository>,
}

impl App {
    pub fn new(
        service: TrainerService<SqliteRepository>,
        language: UiLanguage,
    ) -> Result<Self, RepositoryError> {
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

            language,

            home_selected: 0,

            topic_selected: 0,

            course_selected: 0,

            limit_selected: 0,

            option_selected: 0,

            topics,

            quiz_scope: QuizScope::AllTopics,

            current_question: None,

            selected_answer_ids: Vec::new(),

            feedback: None,

            feedback_scroll: 0,

            answered_questions: 0,

            correct_answers: 0,

            total_questions: 0,

            error_message: None,

            session_id: None,

            resume_session,

            decision_selected: 0,

            statistics: None,

            statistics_view: StatisticsView::Root,

            statistics_selected: 0,

            statistics_course_summaries: Vec::new(),

            statistics_topic_summaries: Vec::new(),

            statistics_mode_summaries: Vec::new(),

            statistics_filter_summaries: Vec::new(),

            statistics_history: Vec::new(),

            cancel_return_screen: Screen::Quiz,

            service,
        })
    }

    pub fn strings(&self) -> &'static UiStrings {
        ui_strings(self.language)
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

            Screen::Courses => self.handle_courses_action(action),

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
                    self.course_selected = 0;
                    self.screen = Screen::Courses;
                }

                HomeItem::Statistics => {
                    self.statistics_view = StatisticsView::Root;
                    self.statistics_selected = 0;
                    self.statistics = None;
                    self.statistics_history.clear();
                    self.screen = Screen::Statistics;
                }

                HomeItem::Language => {
                    self.language = self.language.toggle();
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

    fn handle_courses_action(&mut self, action: Action) -> Result<(), RepositoryError> {
        let courses = self.course_codes();

        match action {
            Action::Up => {
                self.course_selected = previous_index(self.course_selected, courses.len());
            }

            Action::Down => {
                self.course_selected = next_index(self.course_selected, courses.len());
            }

            Action::Confirm => {
                if let Some(course_code) = courses.get(self.course_selected) {
                    self.quiz_scope = QuizScope::Course(course_code.clone());
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
                self.screen = match &self.quiz_scope {
                    QuizScope::Topic(_) => Screen::Topics,
                    QuizScope::Course(_) => Screen::Courses,
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
            Action::Up => {
                self.feedback_scroll = self.feedback_scroll.saturating_sub(1);
            }

            Action::Down => {
                self.feedback_scroll = self.feedback_scroll.saturating_add(1);
            }

            Action::Confirm => {
                self.feedback_scroll = 0;

                self.load_current_question()?;
            }

            Action::Back => {
                self.cancel_return_screen = Screen::Feedback;

                self.decision_selected = 0;

                self.screen = Screen::CancelSession;
            }

            Action::Toggle | Action::ClearStatistics | Action::Quit => {}
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
            Action::Up => {
                let count = self.statistics_item_count();
                self.statistics_selected = previous_index(self.statistics_selected, count);
            }

            Action::Down => {
                let count = self.statistics_item_count();
                self.statistics_selected = next_index(self.statistics_selected, count);
            }

            Action::Confirm => {
                self.confirm_statistics_selection()?;
            }

            Action::Back => {
                self.back_statistics();
            }

            Action::ClearStatistics => {
                self.service.clear_statistics()?;
                self.zero_statistics_summaries();
                self.refresh_statistics_detail()?;
            }

            Action::Toggle | Action::Quit => {}
        }

        Ok(())
    }

    fn confirm_statistics_selection(&mut self) -> Result<(), RepositoryError> {
        match self.statistics_view.clone() {
            StatisticsView::Root => match self.statistics_selected {
                0 => {
                    let title = self.strings().overall_statistics.to_string();
                    self.open_statistics_detail(title, StatisticsFilter::all())?;
                }
                1 => {
                    self.load_statistics_course_summaries()?;
                    self.open_statistics_view(StatisticsView::Courses);
                }
                2 => {
                    self.load_statistics_mode_summaries()?;
                    self.open_statistics_view(StatisticsView::Modes);
                }
                _ => {}
            },

            StatisticsView::Courses => {
                let course_codes = self.statistics_course_codes();

                if let Some(course_code) = course_codes.get(self.statistics_selected) {
                    let course_code = course_code.clone();

                    self.load_statistics_topic_summaries(&course_code)?;
                    self.open_statistics_view(StatisticsView::Course(course_code));
                }
            }

            StatisticsView::Course(course_code) => {
                if self.statistics_selected == 0 {
                    let scope = StatisticsScope::Course(course_code.clone());

                    self.load_statistics_filter_summaries(&scope)?;
                    self.open_statistics_view(StatisticsView::Filters {
                        scope,
                        title: course_code.to_uppercase(),
                    });
                } else {
                    let topic_index = self.statistics_selected - 1;
                    let selected_topic = self
                        .statistics_topics_for_course(&course_code)
                        .get(topic_index)
                        .map(|topic| (topic.id, topic.title.clone()));

                    if let Some((topic_id, topic_title)) = selected_topic {
                        let scope = StatisticsScope::Topic(topic_id);

                        self.load_statistics_filter_summaries(&scope)?;
                        self.open_statistics_view(StatisticsView::Filters {
                            scope,
                            title: format!("{} → {}", course_code.to_uppercase(), topic_title),
                        });
                    }
                }
            }

            StatisticsView::Filters { scope, title } => {
                let limit = statistics_limit_from_index(self.statistics_selected);

                if let Some(limit) = limit {
                    let detail_title =
                        format!("{} → {}", title, self.strings().statistics_limit(limit));

                    self.open_statistics_detail(detail_title, StatisticsFilter { scope, limit })?;
                }
            }

            StatisticsView::Modes => {
                let limit = match self.statistics_selected {
                    0 => Some(StatisticsLimit::Twenty),
                    1 => Some(StatisticsLimit::Fifty),
                    2 => Some(StatisticsLimit::AllQuestions),
                    _ => None,
                };

                if let Some(limit) = limit {
                    let title = format!(
                        "{} → {}",
                        self.strings().all_courses,
                        self.strings().statistics_limit(limit),
                    );

                    self.open_statistics_detail(
                        title,
                        StatisticsFilter {
                            scope: StatisticsScope::All,
                            limit,
                        },
                    )?;
                }
            }

            StatisticsView::Detail { .. } => {
                self.back_statistics();
            }
        }

        Ok(())
    }

    fn open_statistics_view(&mut self, view: StatisticsView) {
        let previous = std::mem::replace(&mut self.statistics_view, view);

        self.statistics_history
            .push((previous, self.statistics_selected));

        self.statistics_selected = 0;
        self.statistics = None;
    }

    fn open_statistics_detail(
        &mut self,
        title: String,
        filter: StatisticsFilter,
    ) -> Result<(), RepositoryError> {
        let stats = self.service.statistics(&filter)?;

        self.open_statistics_view(StatisticsView::Detail { filter, title });
        self.statistics = Some(stats);

        Ok(())
    }

    fn back_statistics(&mut self) {
        if let Some((view, selected)) = self.statistics_history.pop() {
            self.statistics_view = view;
            self.statistics_selected = selected;
            self.statistics = None;
        } else {
            self.statistics_view = StatisticsView::Root;
            self.statistics_selected = 0;
            self.statistics = None;
            self.screen = Screen::Home;
        }
    }

    fn refresh_statistics_detail(&mut self) -> Result<(), RepositoryError> {
        let filter = match &self.statistics_view {
            StatisticsView::Detail { filter, .. } => filter.clone(),
            _ => {
                self.statistics = None;
                return Ok(());
            }
        };

        self.statistics = Some(self.service.statistics(&filter)?);

        Ok(())
    }

    fn statistics_item_count(&self) -> usize {
        match &self.statistics_view {
            StatisticsView::Root => 3,
            StatisticsView::Courses => self.statistics_course_codes().len(),
            StatisticsView::Course(course_code) => {
                1 + self.statistics_topics_for_course(course_code).len()
            }
            StatisticsView::Filters { .. } => 4,
            StatisticsView::Modes => 3,
            StatisticsView::Detail { .. } => 0,
        }
    }

    pub fn course_codes(&self) -> Vec<String> {
        let mut course_codes = self
            .topics
            .iter()
            .map(|topic| topic.course_code.clone())
            .collect::<Vec<_>>();

        course_codes.sort();
        course_codes.dedup();
        course_codes
    }

    pub fn statistics_course_codes(&self) -> Vec<String> {
        self.course_codes()
    }

    pub fn statistics_topics_for_course(&self, course_code: &str) -> Vec<&Topic> {
        self.topics
            .iter()
            .filter(|topic| topic.course_code == course_code)
            .collect()
    }

    pub fn statistics_for_course(&self, course_code: &str) -> Option<&TrainingStats> {
        self.statistics_course_summaries
            .iter()
            .find(|(code, _)| code == course_code)
            .map(|(_, stats)| stats)
    }

    pub fn statistics_for_topic(&self, topic_id: TopicId) -> Option<&TrainingStats> {
        self.statistics_topic_summaries
            .iter()
            .find(|(id, _)| *id == topic_id)
            .map(|(_, stats)| stats)
    }

    pub fn statistics_for_mode(&self, limit: StatisticsLimit) -> Option<&TrainingStats> {
        self.statistics_mode_summaries
            .iter()
            .find(|(mode, _)| *mode == limit)
            .map(|(_, stats)| stats)
    }

    pub fn statistics_for_filter_limit(&self, limit: StatisticsLimit) -> Option<&TrainingStats> {
        self.statistics_filter_summaries
            .iter()
            .find(|(mode, _)| *mode == limit)
            .map(|(_, stats)| stats)
    }

    fn load_statistics_course_summaries(&mut self) -> Result<(), RepositoryError> {
        let course_codes = self.statistics_course_codes();
        let mut summaries = Vec::with_capacity(course_codes.len());

        for course_code in course_codes {
            let stats = self.service.statistics(&StatisticsFilter {
                scope: StatisticsScope::Course(course_code.clone()),
                limit: StatisticsLimit::Any,
            })?;

            summaries.push((course_code, stats));
        }

        self.statistics_course_summaries = summaries;

        Ok(())
    }

    fn load_statistics_topic_summaries(
        &mut self,
        course_code: &str,
    ) -> Result<(), RepositoryError> {
        let topic_ids = self
            .statistics_topics_for_course(course_code)
            .into_iter()
            .map(|topic| topic.id)
            .collect::<Vec<_>>();

        let mut summaries = Vec::with_capacity(topic_ids.len());

        for topic_id in topic_ids {
            let stats = self.service.statistics(&StatisticsFilter {
                scope: StatisticsScope::Topic(topic_id),
                limit: StatisticsLimit::Any,
            })?;

            summaries.push((topic_id, stats));
        }

        self.statistics_topic_summaries = summaries;

        Ok(())
    }

    fn load_statistics_mode_summaries(&mut self) -> Result<(), RepositoryError> {
        let limits = [
            StatisticsLimit::Twenty,
            StatisticsLimit::Fifty,
            StatisticsLimit::AllQuestions,
        ];

        let mut summaries = Vec::with_capacity(limits.len());

        for limit in limits {
            let stats = self.service.statistics(&StatisticsFilter {
                scope: StatisticsScope::All,
                limit,
            })?;

            summaries.push((limit, stats));
        }

        self.statistics_mode_summaries = summaries;

        Ok(())
    }

    fn load_statistics_filter_summaries(
        &mut self,
        scope: &StatisticsScope,
    ) -> Result<(), RepositoryError> {
        let limits = [
            StatisticsLimit::Any,
            StatisticsLimit::Twenty,
            StatisticsLimit::Fifty,
            StatisticsLimit::AllQuestions,
        ];

        let mut summaries = Vec::with_capacity(limits.len());

        for limit in limits {
            let stats = self.service.statistics(&StatisticsFilter {
                scope: scope.clone(),
                limit,
            })?;

            summaries.push((limit, stats));
        }

        self.statistics_filter_summaries = summaries;

        Ok(())
    }

    fn zero_statistics_summaries(&mut self) {
        for (_, stats) in &mut self.statistics_course_summaries {
            *stats = TrainingStats::default();
        }

        for (_, stats) in &mut self.statistics_topic_summaries {
            *stats = TrainingStats::default();
        }

        for (_, stats) in &mut self.statistics_mode_summaries {
            *stats = TrainingStats::default();
        }

        for (_, stats) in &mut self.statistics_filter_summaries {
            *stats = TrainingStats::default();
        }
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
            scope: self.quiz_scope.clone(),

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
                    let message = self.strings().select_at_least_one.to_string();
                    self.error_message = Some(message);

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

        self.feedback_scroll = 0;

        self.feedback = Some(result);

        self.screen = Screen::Feedback;

        Ok(())
    }

    fn reset_quiz(&mut self) {
        self.session_id = None;

        self.current_question = None;

        self.feedback_scroll = 0;

        self.feedback = None;

        self.selected_answer_ids.clear();

        self.option_selected = 0;

        self.answered_questions = 0;

        self.correct_answers = 0;

        self.total_questions = 0;

        self.error_message = None;
    }
}

fn statistics_limit_from_index(index: usize) -> Option<StatisticsLimit> {
    match index {
        0 => Some(StatisticsLimit::Any),
        1 => Some(StatisticsLimit::Twenty),
        2 => Some(StatisticsLimit::Fifty),
        3 => Some(StatisticsLimit::AllQuestions),
        _ => None,
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
