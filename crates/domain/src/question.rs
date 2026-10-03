use crate::{AnswerOptionId, QuestionId, Source, TopicId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerOption {
    pub id: AnswerOptionId,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub topic_id: TopicId,
    pub text: String,
    pub explanation: String,
    pub source: Source,
    pub options: Vec<AnswerOption>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceId;

    #[test]
    fn question_can_contain_answer_options() {
        let question = Question {
            id: QuestionId(1),
            topic_id: TopicId(1),
            text: String::from("Test question"),
            explanation: String::from("test explanation"),
            source: Source {
                id: SourceId(1),
                module: String::from("DBA-1"),
                section: String::from("Test section"),
                locator: String::from("page 1"),
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
