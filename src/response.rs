use serde::Serialize;
use chrono::{Utc, DateTime};

/// Structure de métadonnées optionnelle pour les réponses
#[derive(Serialize)]
pub struct Meta {
    pub timestamp: DateTime<Utc>,
    pub count: Option<usize>, // nombre d'éléments retournés
    pub page: Option<usize>,  // pour la pagination
    pub per_page: Option<usize>,
}

/// Structure principale de la réponse API
#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub meta: Option<Meta>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Réponse réussie avec données
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            meta: Some(Meta {
                timestamp: Utc::now(),
                count: Some(1),
                page: None,
                per_page: None,
            }),
        }
    }

    /// Réponse réussie avec message seulement
    pub fn success_message(msg: &str) -> Self {
        Self {
            success: true,
            data: None,
            message: Some(msg.to_string()),
            meta: Some(Meta {
                timestamp: Utc::now(),
                count: None,
                page: None,
                per_page: None,
            }),
        }
    }

    /// Réponse réussie avec données et métadonnées personnalisées
    pub fn success_with_meta(data: T, count: usize, page: Option<usize>, per_page: Option<usize>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            meta: Some(Meta {
                timestamp: Utc::now(),
                count: Some(count),
                page,
                per_page,
            }),
        }
    }

    /// Réponse d'erreur
    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(msg.to_string()),
            meta: Some(Meta {
                timestamp: Utc::now(),
                count: None,
                page: None,
                per_page: None,
            }),
        }
    }
}
