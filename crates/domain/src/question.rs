use crate::{AnswerOptionId, QuestionId, Source, TopicId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerOption {
    pub id: AnswerOptionId,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub topic_id: TopicId,

    pub question_type: QuestionType,

    pub text: String,
    pub explanation: String,

    pub source: Source,
    pub options: Vec<AnswerOption>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SourceId, SourceKind};

    #[test]
    fn question_can_contain_answer_options() {
        let question = Question {
            id: QuestionId(1),
            topic_id: TopicId(1),

            question_type: QuestionType::SingleChoice,

            text: String::from("Test question"),
            explanation: String::from("test explanation"),

            source: Source {
                id: SourceId(1),
                kind: SourceKind::CourseMaterial,
                module: String::from("DBA-1"),
                section: String::from("Test section"),
                locator: String::from("page 1"),
                url: None,
            },
            options: vec![
                AnswerOption {
                    id: AnswerOptionId(1),
                    text: String::from("Answer A"),
                    is_correct: true,
                },
                AnswerOption {
                    id: AnswerOptionId(1),
                    text: String::from("Answer A"),
                    is_correct: true,
                },
            ],
        };

        assert_eq!(question.options.len(), 2)
    }
}
