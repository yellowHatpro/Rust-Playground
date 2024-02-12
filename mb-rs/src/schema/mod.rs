#![allow(dead_code)]
// Generated with sql-gen
//https://github.com/jayy-lmao/sql-gen

pub mod alternative_medium;
pub use alternative_medium::AlternativeMedium;
pub mod alternative_medium_db_set;
pub use alternative_medium_db_set::AlternativeMediumSet;

pub mod alternative_medium_track;
pub use alternative_medium_track::AlternativeMediumTrack;
pub mod alternative_medium_track_db_set;
pub use alternative_medium_track_db_set::AlternativeMediumTrackSet;

pub mod alternative_release;
pub use alternative_release::AlternativeRelease;
pub mod alternative_release_db_set;
pub use alternative_release_db_set::AlternativeReleaseSet;

pub mod alternative_release_type;
pub use alternative_release_type::AlternativeReleaseType;
pub mod alternative_release_type_db_set;
pub use alternative_release_type_db_set::AlternativeReleaseTypeSet;

pub mod alternative_track;
pub use alternative_track::AlternativeTrack;
pub mod alternative_track_db_set;
pub use alternative_track_db_set::AlternativeTrackSet;

pub mod annotation;
pub use annotation::Annotation;
pub mod annotation_db_set;
pub use annotation_db_set::AnnotationSet;

pub mod application;
pub use application::Application;
pub mod application_db_set;
pub use application_db_set::ApplicationSet;

pub mod area;
pub use area::Area;
pub mod area_db_set;
pub use area_db_set::AreaSet;

pub mod area_alias;
pub use area_alias::AreaAlias;
pub mod area_alias_db_set;
pub use area_alias_db_set::AreaAliasSet;

pub mod area_alias_type;
pub use area_alias_type::AreaAliasType;
pub mod area_alias_type_db_set;
pub use area_alias_type_db_set::AreaAliasTypeSet;

pub mod area_annotation;
pub use area_annotation::AreaAnnotation;
pub mod area_annotation_db_set;
pub use area_annotation_db_set::AreaAnnotationSet;

pub mod area_attribute;
pub use area_attribute::AreaAttribute;
pub mod area_attribute_db_set;
pub use area_attribute_db_set::AreaAttributeSet;

pub mod area_attribute_type;
pub use area_attribute_type::AreaAttributeType;
pub mod area_attribute_type_db_set;
pub use area_attribute_type_db_set::AreaAttributeTypeSet;

pub mod area_attribute_type_allowed_value;
pub use area_attribute_type_allowed_value::AreaAttributeTypeAllowedValue;
pub mod area_attribute_type_allowed_value_db_set;
pub use area_attribute_type_allowed_value_db_set::AreaAttributeTypeAllowedValueSet;

pub mod area_containment;
pub use area_containment::AreaContainment;
pub mod area_containment_db_set;
pub use area_containment_db_set::AreaContainmentSet;

pub mod area_gid_redirect;
pub use area_gid_redirect::AreaGidRedirect;
pub mod area_gid_redirect_db_set;
pub use area_gid_redirect_db_set::AreaGidRedirectSet;

pub mod area_tag;
pub use area_tag::AreaTag;
pub mod area_tag_db_set;
pub use area_tag_db_set::AreaTagSet;

pub mod area_tag_raw;
pub use area_tag_raw::AreaTagRaw;
pub mod area_tag_raw_db_set;
pub use area_tag_raw_db_set::AreaTagRawSet;

pub mod area_type;
pub use area_type::AreaType;
pub mod area_type_db_set;
pub use area_type_db_set::AreaTypeSet;

pub mod artist;
pub use artist::Artist;
pub mod artist_db_set;
pub use artist_db_set::ArtistSet;

pub mod artist_alias;
pub use artist_alias::ArtistAlias;
pub mod artist_alias_db_set;
pub use artist_alias_db_set::ArtistAliasSet;

pub mod artist_alias_type;
pub use artist_alias_type::ArtistAliasType;
pub mod artist_alias_type_db_set;
pub use artist_alias_type_db_set::ArtistAliasTypeSet;

pub mod artist_annotation;
pub use artist_annotation::ArtistAnnotation;
pub mod artist_annotation_db_set;
pub use artist_annotation_db_set::ArtistAnnotationSet;

pub mod artist_attribute;
pub use artist_attribute::ArtistAttribute;
pub mod artist_attribute_db_set;
pub use artist_attribute_db_set::ArtistAttributeSet;

pub mod artist_attribute_type;
pub use artist_attribute_type::ArtistAttributeType;
pub mod artist_attribute_type_db_set;
pub use artist_attribute_type_db_set::ArtistAttributeTypeSet;

pub mod artist_attribute_type_allowed_value;
pub use artist_attribute_type_allowed_value::ArtistAttributeTypeAllowedValue;
pub mod artist_attribute_type_allowed_value_db_set;
pub use artist_attribute_type_allowed_value_db_set::ArtistAttributeTypeAllowedValueSet;

pub mod artist_credit;
pub use artist_credit::ArtistCredit;
pub mod artist_credit_db_set;
pub use artist_credit_db_set::ArtistCreditSet;

pub mod artist_credit_gid_redirect;
pub use artist_credit_gid_redirect::ArtistCreditGidRedirect;
pub mod artist_credit_gid_redirect_db_set;
pub use artist_credit_gid_redirect_db_set::ArtistCreditGidRedirectSet;

pub mod artist_credit_name;
pub use artist_credit_name::ArtistCreditName;
pub mod artist_credit_name_db_set;
pub use artist_credit_name_db_set::ArtistCreditNameSet;

pub mod artist_gid_redirect;
pub use artist_gid_redirect::ArtistGidRedirect;
pub mod artist_gid_redirect_db_set;
pub use artist_gid_redirect_db_set::ArtistGidRedirectSet;

pub mod artist_ipi;
pub use artist_ipi::ArtistIpi;
pub mod artist_ipi_db_set;
pub use artist_ipi_db_set::ArtistIpiSet;

pub mod artist_isni;
pub use artist_isni::ArtistIsni;
pub mod artist_isni_db_set;
pub use artist_isni_db_set::ArtistIsniSet;

pub mod artist_meta;
pub use artist_meta::ArtistMeta;
pub mod artist_meta_db_set;
pub use artist_meta_db_set::ArtistMetaSet;

pub mod artist_rating_raw;
pub use artist_rating_raw::ArtistRatingRaw;
pub mod artist_rating_raw_db_set;
pub use artist_rating_raw_db_set::ArtistRatingRawSet;

pub mod artist_release;
pub use artist_release::ArtistRelease;
pub mod artist_release_db_set;
pub use artist_release_db_set::ArtistReleaseSet;

pub mod artist_release_group;
pub use artist_release_group::ArtistReleaseGroup;
pub mod artist_release_group_db_set;
pub use artist_release_group_db_set::ArtistReleaseGroupSet;

pub mod artist_release_group_nonva;
pub use artist_release_group_nonva::ArtistReleaseGroupNonva;
pub mod artist_release_group_nonva_db_set;
pub use artist_release_group_nonva_db_set::ArtistReleaseGroupNonvaSet;

pub mod artist_release_group_pending_update;
pub use artist_release_group_pending_update::ArtistReleaseGroupPendingUpdate;
pub mod artist_release_group_pending_update_db_set;
pub use artist_release_group_pending_update_db_set::ArtistReleaseGroupPendingUpdateSet;

pub mod artist_release_group_va;
pub use artist_release_group_va::ArtistReleaseGroupVa;
pub mod artist_release_group_va_db_set;
pub use artist_release_group_va_db_set::ArtistReleaseGroupVaSet;

pub mod artist_release_nonva;
pub use artist_release_nonva::ArtistReleaseNonva;
pub mod artist_release_nonva_db_set;
pub use artist_release_nonva_db_set::ArtistReleaseNonvaSet;

pub mod artist_release_pending_update;
pub use artist_release_pending_update::ArtistReleasePendingUpdate;
pub mod artist_release_pending_update_db_set;
pub use artist_release_pending_update_db_set::ArtistReleasePendingUpdateSet;

pub mod artist_release_va;
pub use artist_release_va::ArtistReleaseVa;
pub mod artist_release_va_db_set;
pub use artist_release_va_db_set::ArtistReleaseVaSet;

pub mod artist_series;
pub use artist_series::ArtistSeries;
pub mod artist_series_db_set;
pub use artist_series_db_set::ArtistSeriesSet;

pub mod artist_tag;
pub use artist_tag::ArtistTag;
pub mod artist_tag_db_set;
pub use artist_tag_db_set::ArtistTagSet;

pub mod artist_tag_raw;
pub use artist_tag_raw::ArtistTagRaw;
pub mod artist_tag_raw_db_set;
pub use artist_tag_raw_db_set::ArtistTagRawSet;

pub mod artist_type;
pub use artist_type::ArtistType;
pub mod artist_type_db_set;
pub use artist_type_db_set::ArtistTypeSet;

pub mod autoeditor_election;
pub use autoeditor_election::AutoeditorElection;
pub mod autoeditor_election_db_set;
pub use autoeditor_election_db_set::AutoeditorElectionSet;

pub mod autoeditor_election_vote;
pub use autoeditor_election_vote::AutoeditorElectionVote;
pub mod autoeditor_election_vote_db_set;
pub use autoeditor_election_vote_db_set::AutoeditorElectionVoteSet;

pub mod cdtoc;
pub use cdtoc::Cdtoc;
pub mod cdtoc_db_set;
pub use cdtoc_db_set::CdtocSet;

pub mod cdtoc_raw;
pub use cdtoc_raw::CdtocRaw;
pub mod cdtoc_raw_db_set;
pub use cdtoc_raw_db_set::CdtocRawSet;

pub mod country_area;
pub use country_area::CountryArea;
pub mod country_area_db_set;
pub use country_area_db_set::CountryAreaSet;

pub mod cube_index;
pub use cube_index::CubeIndex;
pub mod cube_index_db_set;
pub use cube_index_db_set::CubeIndexSet;

pub mod deleted_entity;
pub use deleted_entity::DeletedEntity;
pub mod deleted_entity_db_set;
pub use deleted_entity_db_set::DeletedEntitySet;

pub mod edit;
pub use edit::Edit;
pub mod edit_db_set;
pub use edit_db_set::EditSet;

pub mod edit_area;
pub use edit_area::EditArea;
pub mod edit_area_db_set;
pub use edit_area_db_set::EditAreaSet;

pub mod edit_artist;
pub use edit_artist::EditArtist;
pub mod edit_artist_db_set;
pub use edit_artist_db_set::EditArtistSet;

pub mod edit_data;
pub use edit_data::EditData;
pub mod edit_data_db_set;
pub use edit_data_db_set::EditDataSet;

pub mod edit_event;
pub use edit_event::EditEvent;
pub mod edit_event_db_set;
pub use edit_event_db_set::EditEventSet;

pub mod edit_genre;
pub use edit_genre::EditGenre;
pub mod edit_genre_db_set;
pub use edit_genre_db_set::EditGenreSet;

pub mod edit_instrument;
pub use edit_instrument::EditInstrument;
pub mod edit_instrument_db_set;
pub use edit_instrument_db_set::EditInstrumentSet;

pub mod edit_label;
pub use edit_label::EditLabel;
pub mod edit_label_db_set;
pub use edit_label_db_set::EditLabelSet;

pub mod edit_mood;
pub use edit_mood::EditMood;
pub mod edit_mood_db_set;
pub use edit_mood_db_set::EditMoodSet;

pub mod edit_note;
pub use edit_note::EditNote;
pub mod edit_note_db_set;
pub use edit_note_db_set::EditNoteSet;

pub mod edit_note_change;
pub use edit_note_change::EditNoteChange;
pub mod edit_note_change_db_set;
pub use edit_note_change_db_set::EditNoteChangeSet;

pub mod edit_note_recipient;
pub use edit_note_recipient::EditNoteRecipient;
pub mod edit_note_recipient_db_set;
pub use edit_note_recipient_db_set::EditNoteRecipientSet;

pub mod edit_place;
pub use edit_place::EditPlace;
pub mod edit_place_db_set;
pub use edit_place_db_set::EditPlaceSet;

pub mod edit_recording;
pub use edit_recording::EditRecording;
pub mod edit_recording_db_set;
pub use edit_recording_db_set::EditRecordingSet;

pub mod edit_release;
pub use edit_release::EditRelease;
pub mod edit_release_db_set;
pub use edit_release_db_set::EditReleaseSet;

pub mod edit_release_group;
pub use edit_release_group::EditReleaseGroup;
pub mod edit_release_group_db_set;
pub use edit_release_group_db_set::EditReleaseGroupSet;

pub mod edit_series;
pub use edit_series::EditSeries;
pub mod edit_series_db_set;
pub use edit_series_db_set::EditSeriesSet;

pub mod edit_url;
pub use edit_url::EditUrl;
pub mod edit_url_db_set;
pub use edit_url_db_set::EditUrlSet;

pub mod edit_work;
pub use edit_work::EditWork;
pub mod edit_work_db_set;
pub use edit_work_db_set::EditWorkSet;

pub mod editor;
pub use editor::Editor;
pub mod editor_db_set;
pub use editor_db_set::EditorSet;

pub mod editor_collection;
pub use editor_collection::EditorCollection;
pub mod editor_collection_db_set;
pub use editor_collection_db_set::EditorCollectionSet;

pub mod editor_collection_area;
pub use editor_collection_area::EditorCollectionArea;
pub mod editor_collection_area_db_set;
pub use editor_collection_area_db_set::EditorCollectionAreaSet;

pub mod editor_collection_artist;
pub use editor_collection_artist::EditorCollectionArtist;
pub mod editor_collection_artist_db_set;
pub use editor_collection_artist_db_set::EditorCollectionArtistSet;

pub mod editor_collection_collaborator;
pub use editor_collection_collaborator::EditorCollectionCollaborator;
pub mod editor_collection_collaborator_db_set;
pub use editor_collection_collaborator_db_set::EditorCollectionCollaboratorSet;

pub mod editor_collection_deleted_entity;
pub use editor_collection_deleted_entity::EditorCollectionDeletedEntity;
pub mod editor_collection_deleted_entity_db_set;
pub use editor_collection_deleted_entity_db_set::EditorCollectionDeletedEntitySet;

pub mod editor_collection_event;
pub use editor_collection_event::EditorCollectionEvent;
pub mod editor_collection_event_db_set;
pub use editor_collection_event_db_set::EditorCollectionEventSet;

pub mod editor_collection_gid_redirect;
pub use editor_collection_gid_redirect::EditorCollectionGidRedirect;
pub mod editor_collection_gid_redirect_db_set;
pub use editor_collection_gid_redirect_db_set::EditorCollectionGidRedirectSet;

pub mod editor_collection_instrument;
pub use editor_collection_instrument::EditorCollectionInstrument;
pub mod editor_collection_instrument_db_set;
pub use editor_collection_instrument_db_set::EditorCollectionInstrumentSet;

pub mod editor_collection_label;
pub use editor_collection_label::EditorCollectionLabel;
pub mod editor_collection_label_db_set;
pub use editor_collection_label_db_set::EditorCollectionLabelSet;

pub mod editor_collection_place;
pub use editor_collection_place::EditorCollectionPlace;
pub mod editor_collection_place_db_set;
pub use editor_collection_place_db_set::EditorCollectionPlaceSet;

pub mod editor_collection_recording;
pub use editor_collection_recording::EditorCollectionRecording;
pub mod editor_collection_recording_db_set;
pub use editor_collection_recording_db_set::EditorCollectionRecordingSet;

pub mod editor_collection_release;
pub use editor_collection_release::EditorCollectionRelease;
pub mod editor_collection_release_db_set;
pub use editor_collection_release_db_set::EditorCollectionReleaseSet;

pub mod editor_collection_release_group;
pub use editor_collection_release_group::EditorCollectionReleaseGroup;
pub mod editor_collection_release_group_db_set;
pub use editor_collection_release_group_db_set::EditorCollectionReleaseGroupSet;

pub mod editor_collection_series;
pub use editor_collection_series::EditorCollectionSeries;
pub mod editor_collection_series_db_set;
pub use editor_collection_series_db_set::EditorCollectionSeriesSet;

pub mod editor_collection_type;
pub use editor_collection_type::EditorCollectionType;
pub mod editor_collection_type_db_set;
pub use editor_collection_type_db_set::EditorCollectionTypeSet;

pub mod editor_collection_work;
pub use editor_collection_work::EditorCollectionWork;
pub mod editor_collection_work_db_set;
pub use editor_collection_work_db_set::EditorCollectionWorkSet;

pub mod editor_language;
pub use editor_language::EditorLanguage;
pub mod editor_language_db_set;
pub use editor_language_db_set::EditorLanguageSet;

pub mod editor_oauth_token;
pub use editor_oauth_token::EditorOauthToken;
pub mod editor_oauth_token_db_set;
pub use editor_oauth_token_db_set::EditorOauthTokenSet;

pub mod editor_preference;
pub use editor_preference::EditorPreference;
pub mod editor_preference_db_set;
pub use editor_preference_db_set::EditorPreferenceSet;

pub mod editor_subscribe_artist;
pub use editor_subscribe_artist::EditorSubscribeArtist;
pub mod editor_subscribe_artist_db_set;
pub use editor_subscribe_artist_db_set::EditorSubscribeArtistSet;

pub mod editor_subscribe_artist_deleted;
pub use editor_subscribe_artist_deleted::EditorSubscribeArtistDeleted;
pub mod editor_subscribe_artist_deleted_db_set;
pub use editor_subscribe_artist_deleted_db_set::EditorSubscribeArtistDeletedSet;

pub mod editor_subscribe_collection;
pub use editor_subscribe_collection::EditorSubscribeCollection;
pub mod editor_subscribe_collection_db_set;
pub use editor_subscribe_collection_db_set::EditorSubscribeCollectionSet;

pub mod editor_subscribe_editor;
pub use editor_subscribe_editor::EditorSubscribeEditor;
pub mod editor_subscribe_editor_db_set;
pub use editor_subscribe_editor_db_set::EditorSubscribeEditorSet;

pub mod editor_subscribe_label;
pub use editor_subscribe_label::EditorSubscribeLabel;
pub mod editor_subscribe_label_db_set;
pub use editor_subscribe_label_db_set::EditorSubscribeLabelSet;

pub mod editor_subscribe_label_deleted;
pub use editor_subscribe_label_deleted::EditorSubscribeLabelDeleted;
pub mod editor_subscribe_label_deleted_db_set;
pub use editor_subscribe_label_deleted_db_set::EditorSubscribeLabelDeletedSet;

pub mod editor_subscribe_series;
pub use editor_subscribe_series::EditorSubscribeSeries;
pub mod editor_subscribe_series_db_set;
pub use editor_subscribe_series_db_set::EditorSubscribeSeriesSet;

pub mod editor_subscribe_series_deleted;
pub use editor_subscribe_series_deleted::EditorSubscribeSeriesDeleted;
pub mod editor_subscribe_series_deleted_db_set;
pub use editor_subscribe_series_deleted_db_set::EditorSubscribeSeriesDeletedSet;

pub mod event;
pub use event::Event;
pub mod event_db_set;
pub use event_db_set::EventSet;

pub mod event_alias;
pub use event_alias::EventAlias;
pub mod event_alias_db_set;
pub use event_alias_db_set::EventAliasSet;

pub mod event_alias_type;
pub use event_alias_type::EventAliasType;
pub mod event_alias_type_db_set;
pub use event_alias_type_db_set::EventAliasTypeSet;

pub mod event_annotation;
pub use event_annotation::EventAnnotation;
pub mod event_annotation_db_set;
pub use event_annotation_db_set::EventAnnotationSet;

pub mod event_attribute;
pub use event_attribute::EventAttribute;
pub mod event_attribute_db_set;
pub use event_attribute_db_set::EventAttributeSet;

pub mod event_attribute_type;
pub use event_attribute_type::EventAttributeType;
pub mod event_attribute_type_db_set;
pub use event_attribute_type_db_set::EventAttributeTypeSet;

pub mod event_attribute_type_allowed_value;
pub use event_attribute_type_allowed_value::EventAttributeTypeAllowedValue;
pub mod event_attribute_type_allowed_value_db_set;
pub use event_attribute_type_allowed_value_db_set::EventAttributeTypeAllowedValueSet;

pub mod event_gid_redirect;
pub use event_gid_redirect::EventGidRedirect;
pub mod event_gid_redirect_db_set;
pub use event_gid_redirect_db_set::EventGidRedirectSet;

pub mod event_meta;
pub use event_meta::EventMeta;
pub mod event_meta_db_set;
pub use event_meta_db_set::EventMetaSet;

pub mod event_rating_raw;
pub use event_rating_raw::EventRatingRaw;
pub mod event_rating_raw_db_set;
pub use event_rating_raw_db_set::EventRatingRawSet;

pub mod event_series;
pub use event_series::EventSeries;
pub mod event_series_db_set;
pub use event_series_db_set::EventSeriesSet;

pub mod event_tag;
pub use event_tag::EventTag;
pub mod event_tag_db_set;
pub use event_tag_db_set::EventTagSet;

pub mod event_tag_raw;
pub use event_tag_raw::EventTagRaw;
pub mod event_tag_raw_db_set;
pub use event_tag_raw_db_set::EventTagRawSet;

pub mod event_type;
pub use event_type::EventType;
pub mod event_type_db_set;
pub use event_type_db_set::EventTypeSet;

pub mod gender;
pub use gender::Gender;
pub mod gender_db_set;
pub use gender_db_set::GenderSet;

pub mod genre;
pub use genre::Genre;
pub mod genre_db_set;
pub use genre_db_set::GenreSet;

pub mod genre_alias;
pub use genre_alias::GenreAlias;
pub mod genre_alias_db_set;
pub use genre_alias_db_set::GenreAliasSet;

pub mod genre_alias_type;
pub use genre_alias_type::GenreAliasType;
pub mod genre_alias_type_db_set;
pub use genre_alias_type_db_set::GenreAliasTypeSet;

pub mod genre_annotation;
pub use genre_annotation::GenreAnnotation;
pub mod genre_annotation_db_set;
pub use genre_annotation_db_set::GenreAnnotationSet;

pub mod instrument;
pub use instrument::Instrument;
pub mod instrument_db_set;
pub use instrument_db_set::InstrumentSet;

pub mod instrument_alias;
pub use instrument_alias::InstrumentAlias;
pub mod instrument_alias_db_set;
pub use instrument_alias_db_set::InstrumentAliasSet;

pub mod instrument_alias_type;
pub use instrument_alias_type::InstrumentAliasType;
pub mod instrument_alias_type_db_set;
pub use instrument_alias_type_db_set::InstrumentAliasTypeSet;

pub mod instrument_annotation;
pub use instrument_annotation::InstrumentAnnotation;
pub mod instrument_annotation_db_set;
pub use instrument_annotation_db_set::InstrumentAnnotationSet;

pub mod instrument_attribute;
pub use instrument_attribute::InstrumentAttribute;
pub mod instrument_attribute_db_set;
pub use instrument_attribute_db_set::InstrumentAttributeSet;

pub mod instrument_attribute_type;
pub use instrument_attribute_type::InstrumentAttributeType;
pub mod instrument_attribute_type_db_set;
pub use instrument_attribute_type_db_set::InstrumentAttributeTypeSet;

pub mod instrument_attribute_type_allowed_value;
pub use instrument_attribute_type_allowed_value::InstrumentAttributeTypeAllowedValue;
pub mod instrument_attribute_type_allowed_value_db_set;
pub use instrument_attribute_type_allowed_value_db_set::InstrumentAttributeTypeAllowedValueSet;

pub mod instrument_gid_redirect;
pub use instrument_gid_redirect::InstrumentGidRedirect;
pub mod instrument_gid_redirect_db_set;
pub use instrument_gid_redirect_db_set::InstrumentGidRedirectSet;

pub mod instrument_tag;
pub use instrument_tag::InstrumentTag;
pub mod instrument_tag_db_set;
pub use instrument_tag_db_set::InstrumentTagSet;

pub mod instrument_tag_raw;
pub use instrument_tag_raw::InstrumentTagRaw;
pub mod instrument_tag_raw_db_set;
pub use instrument_tag_raw_db_set::InstrumentTagRawSet;

pub mod instrument_type;
pub use instrument_type::InstrumentType;
pub mod instrument_type_db_set;
pub use instrument_type_db_set::InstrumentTypeSet;

pub mod iso_3166_1;
pub use iso_3166_1::Iso31661;
pub mod iso_3166_1_db_set;
pub use iso_3166_1_db_set::Iso31661Set;

pub mod iso_3166_2;
pub use iso_3166_2::Iso31662;
pub mod iso_3166_2_db_set;
pub use iso_3166_2_db_set::Iso31662Set;

pub mod iso_3166_3;
pub use iso_3166_3::Iso31663;
pub mod iso_3166_3_db_set;
pub use iso_3166_3_db_set::Iso31663Set;

pub mod isrc;
pub use isrc::Isrc;
pub mod isrc_db_set;
pub use isrc_db_set::IsrcSet;

pub mod iswc;
pub use iswc::Iswc;
pub mod iswc_db_set;
pub use iswc_db_set::IswcSet;

pub mod l_area_area;
pub use l_area_area::LAreaArea;
pub mod l_area_area_db_set;
pub use l_area_area_db_set::LAreaAreaSet;

pub mod l_area_artist;
pub use l_area_artist::LAreaArtist;
pub mod l_area_artist_db_set;
pub use l_area_artist_db_set::LAreaArtistSet;

pub mod l_area_event;
pub use l_area_event::LAreaEvent;
pub mod l_area_event_db_set;
pub use l_area_event_db_set::LAreaEventSet;

pub mod l_area_genre;
pub use l_area_genre::LAreaGenre;
pub mod l_area_genre_db_set;
pub use l_area_genre_db_set::LAreaGenreSet;

pub mod l_area_instrument;
pub use l_area_instrument::LAreaInstrument;
pub mod l_area_instrument_db_set;
pub use l_area_instrument_db_set::LAreaInstrumentSet;

pub mod l_area_label;
pub use l_area_label::LAreaLabel;
pub mod l_area_label_db_set;
pub use l_area_label_db_set::LAreaLabelSet;

pub mod l_area_mood;
pub use l_area_mood::LAreaMood;
pub mod l_area_mood_db_set;
pub use l_area_mood_db_set::LAreaMoodSet;

pub mod l_area_place;
pub use l_area_place::LAreaPlace;
pub mod l_area_place_db_set;
pub use l_area_place_db_set::LAreaPlaceSet;

pub mod l_area_recording;
pub use l_area_recording::LAreaRecording;
pub mod l_area_recording_db_set;
pub use l_area_recording_db_set::LAreaRecordingSet;

pub mod l_area_release;
pub use l_area_release::LAreaRelease;
pub mod l_area_release_db_set;
pub use l_area_release_db_set::LAreaReleaseSet;

pub mod l_area_release_group;
pub use l_area_release_group::LAreaReleaseGroup;
pub mod l_area_release_group_db_set;
pub use l_area_release_group_db_set::LAreaReleaseGroupSet;

pub mod l_area_series;
pub use l_area_series::LAreaSeries;
pub mod l_area_series_db_set;
pub use l_area_series_db_set::LAreaSeriesSet;

pub mod l_area_url;
pub use l_area_url::LAreaUrl;
pub mod l_area_url_db_set;
pub use l_area_url_db_set::LAreaUrlSet;

pub mod l_area_work;
pub use l_area_work::LAreaWork;
pub mod l_area_work_db_set;
pub use l_area_work_db_set::LAreaWorkSet;

pub mod l_artist_artist;
pub use l_artist_artist::LArtistArtist;
pub mod l_artist_artist_db_set;
pub use l_artist_artist_db_set::LArtistArtistSet;

pub mod l_artist_event;
pub use l_artist_event::LArtistEvent;
pub mod l_artist_event_db_set;
pub use l_artist_event_db_set::LArtistEventSet;

pub mod l_artist_genre;
pub use l_artist_genre::LArtistGenre;
pub mod l_artist_genre_db_set;
pub use l_artist_genre_db_set::LArtistGenreSet;

pub mod l_artist_instrument;
pub use l_artist_instrument::LArtistInstrument;
pub mod l_artist_instrument_db_set;
pub use l_artist_instrument_db_set::LArtistInstrumentSet;

pub mod l_artist_label;
pub use l_artist_label::LArtistLabel;
pub mod l_artist_label_db_set;
pub use l_artist_label_db_set::LArtistLabelSet;

pub mod l_artist_mood;
pub use l_artist_mood::LArtistMood;
pub mod l_artist_mood_db_set;
pub use l_artist_mood_db_set::LArtistMoodSet;

pub mod l_artist_place;
pub use l_artist_place::LArtistPlace;
pub mod l_artist_place_db_set;
pub use l_artist_place_db_set::LArtistPlaceSet;

pub mod l_artist_recording;
pub use l_artist_recording::LArtistRecording;
pub mod l_artist_recording_db_set;
pub use l_artist_recording_db_set::LArtistRecordingSet;

pub mod l_artist_release;
pub use l_artist_release::LArtistRelease;
pub mod l_artist_release_db_set;
pub use l_artist_release_db_set::LArtistReleaseSet;

pub mod l_artist_release_group;
pub use l_artist_release_group::LArtistReleaseGroup;
pub mod l_artist_release_group_db_set;
pub use l_artist_release_group_db_set::LArtistReleaseGroupSet;

pub mod l_artist_series;
pub use l_artist_series::LArtistSeries;
pub mod l_artist_series_db_set;
pub use l_artist_series_db_set::LArtistSeriesSet;

pub mod l_artist_url;
pub use l_artist_url::LArtistUrl;
pub mod l_artist_url_db_set;
pub use l_artist_url_db_set::LArtistUrlSet;

pub mod l_artist_work;
pub use l_artist_work::LArtistWork;
pub mod l_artist_work_db_set;
pub use l_artist_work_db_set::LArtistWorkSet;

pub mod l_event_event;
pub use l_event_event::LEventEvent;
pub mod l_event_event_db_set;
pub use l_event_event_db_set::LEventEventSet;

pub mod l_event_genre;
pub use l_event_genre::LEventGenre;
pub mod l_event_genre_db_set;
pub use l_event_genre_db_set::LEventGenreSet;

pub mod l_event_instrument;
pub use l_event_instrument::LEventInstrument;
pub mod l_event_instrument_db_set;
pub use l_event_instrument_db_set::LEventInstrumentSet;

pub mod l_event_label;
pub use l_event_label::LEventLabel;
pub mod l_event_label_db_set;
pub use l_event_label_db_set::LEventLabelSet;

pub mod l_event_mood;
pub use l_event_mood::LEventMood;
pub mod l_event_mood_db_set;
pub use l_event_mood_db_set::LEventMoodSet;

pub mod l_event_place;
pub use l_event_place::LEventPlace;
pub mod l_event_place_db_set;
pub use l_event_place_db_set::LEventPlaceSet;

pub mod l_event_recording;
pub use l_event_recording::LEventRecording;
pub mod l_event_recording_db_set;
pub use l_event_recording_db_set::LEventRecordingSet;

pub mod l_event_release;
pub use l_event_release::LEventRelease;
pub mod l_event_release_db_set;
pub use l_event_release_db_set::LEventReleaseSet;

pub mod l_event_release_group;
pub use l_event_release_group::LEventReleaseGroup;
pub mod l_event_release_group_db_set;
pub use l_event_release_group_db_set::LEventReleaseGroupSet;

pub mod l_event_series;
pub use l_event_series::LEventSeries;
pub mod l_event_series_db_set;
pub use l_event_series_db_set::LEventSeriesSet;

pub mod l_event_url;
pub use l_event_url::LEventUrl;
pub mod l_event_url_db_set;
pub use l_event_url_db_set::LEventUrlSet;

pub mod l_event_work;
pub use l_event_work::LEventWork;
pub mod l_event_work_db_set;
pub use l_event_work_db_set::LEventWorkSet;

pub mod l_genre_genre;
pub use l_genre_genre::LGenreGenre;
pub mod l_genre_genre_db_set;
pub use l_genre_genre_db_set::LGenreGenreSet;

pub mod l_genre_instrument;
pub use l_genre_instrument::LGenreInstrument;
pub mod l_genre_instrument_db_set;
pub use l_genre_instrument_db_set::LGenreInstrumentSet;

pub mod l_genre_label;
pub use l_genre_label::LGenreLabel;
pub mod l_genre_label_db_set;
pub use l_genre_label_db_set::LGenreLabelSet;

pub mod l_genre_mood;
pub use l_genre_mood::LGenreMood;
pub mod l_genre_mood_db_set;
pub use l_genre_mood_db_set::LGenreMoodSet;

pub mod l_genre_place;
pub use l_genre_place::LGenrePlace;
pub mod l_genre_place_db_set;
pub use l_genre_place_db_set::LGenrePlaceSet;

pub mod l_genre_recording;
pub use l_genre_recording::LGenreRecording;
pub mod l_genre_recording_db_set;
pub use l_genre_recording_db_set::LGenreRecordingSet;

pub mod l_genre_release;
pub use l_genre_release::LGenreRelease;
pub mod l_genre_release_db_set;
pub use l_genre_release_db_set::LGenreReleaseSet;

pub mod l_genre_release_group;
pub use l_genre_release_group::LGenreReleaseGroup;
pub mod l_genre_release_group_db_set;
pub use l_genre_release_group_db_set::LGenreReleaseGroupSet;

pub mod l_genre_series;
pub use l_genre_series::LGenreSeries;
pub mod l_genre_series_db_set;
pub use l_genre_series_db_set::LGenreSeriesSet;

pub mod l_genre_url;
pub use l_genre_url::LGenreUrl;
pub mod l_genre_url_db_set;
pub use l_genre_url_db_set::LGenreUrlSet;

pub mod l_genre_work;
pub use l_genre_work::LGenreWork;
pub mod l_genre_work_db_set;
pub use l_genre_work_db_set::LGenreWorkSet;

pub mod l_instrument_instrument;
pub use l_instrument_instrument::LInstrumentInstrument;
pub mod l_instrument_instrument_db_set;
pub use l_instrument_instrument_db_set::LInstrumentInstrumentSet;

pub mod l_instrument_label;
pub use l_instrument_label::LInstrumentLabel;
pub mod l_instrument_label_db_set;
pub use l_instrument_label_db_set::LInstrumentLabelSet;

pub mod l_instrument_mood;
pub use l_instrument_mood::LInstrumentMood;
pub mod l_instrument_mood_db_set;
pub use l_instrument_mood_db_set::LInstrumentMoodSet;

pub mod l_instrument_place;
pub use l_instrument_place::LInstrumentPlace;
pub mod l_instrument_place_db_set;
pub use l_instrument_place_db_set::LInstrumentPlaceSet;

pub mod l_instrument_recording;
pub use l_instrument_recording::LInstrumentRecording;
pub mod l_instrument_recording_db_set;
pub use l_instrument_recording_db_set::LInstrumentRecordingSet;

pub mod l_instrument_release;
pub use l_instrument_release::LInstrumentRelease;
pub mod l_instrument_release_db_set;
pub use l_instrument_release_db_set::LInstrumentReleaseSet;

pub mod l_instrument_release_group;
pub use l_instrument_release_group::LInstrumentReleaseGroup;
pub mod l_instrument_release_group_db_set;
pub use l_instrument_release_group_db_set::LInstrumentReleaseGroupSet;

pub mod l_instrument_series;
pub use l_instrument_series::LInstrumentSeries;
pub mod l_instrument_series_db_set;
pub use l_instrument_series_db_set::LInstrumentSeriesSet;

pub mod l_instrument_url;
pub use l_instrument_url::LInstrumentUrl;
pub mod l_instrument_url_db_set;
pub use l_instrument_url_db_set::LInstrumentUrlSet;

pub mod l_instrument_work;
pub use l_instrument_work::LInstrumentWork;
pub mod l_instrument_work_db_set;
pub use l_instrument_work_db_set::LInstrumentWorkSet;

pub mod l_label_label;
pub use l_label_label::LLabelLabel;
pub mod l_label_label_db_set;
pub use l_label_label_db_set::LLabelLabelSet;

pub mod l_label_mood;
pub use l_label_mood::LLabelMood;
pub mod l_label_mood_db_set;
pub use l_label_mood_db_set::LLabelMoodSet;

pub mod l_label_place;
pub use l_label_place::LLabelPlace;
pub mod l_label_place_db_set;
pub use l_label_place_db_set::LLabelPlaceSet;

pub mod l_label_recording;
pub use l_label_recording::LLabelRecording;
pub mod l_label_recording_db_set;
pub use l_label_recording_db_set::LLabelRecordingSet;

pub mod l_label_release;
pub use l_label_release::LLabelRelease;
pub mod l_label_release_db_set;
pub use l_label_release_db_set::LLabelReleaseSet;

pub mod l_label_release_group;
pub use l_label_release_group::LLabelReleaseGroup;
pub mod l_label_release_group_db_set;
pub use l_label_release_group_db_set::LLabelReleaseGroupSet;

pub mod l_label_series;
pub use l_label_series::LLabelSeries;
pub mod l_label_series_db_set;
pub use l_label_series_db_set::LLabelSeriesSet;

pub mod l_label_url;
pub use l_label_url::LLabelUrl;
pub mod l_label_url_db_set;
pub use l_label_url_db_set::LLabelUrlSet;

pub mod l_label_work;
pub use l_label_work::LLabelWork;
pub mod l_label_work_db_set;
pub use l_label_work_db_set::LLabelWorkSet;

pub mod l_mood_mood;
pub use l_mood_mood::LMoodMood;
pub mod l_mood_mood_db_set;
pub use l_mood_mood_db_set::LMoodMoodSet;

pub mod l_mood_place;
pub use l_mood_place::LMoodPlace;
pub mod l_mood_place_db_set;
pub use l_mood_place_db_set::LMoodPlaceSet;

pub mod l_mood_recording;
pub use l_mood_recording::LMoodRecording;
pub mod l_mood_recording_db_set;
pub use l_mood_recording_db_set::LMoodRecordingSet;

pub mod l_mood_release;
pub use l_mood_release::LMoodRelease;
pub mod l_mood_release_db_set;
pub use l_mood_release_db_set::LMoodReleaseSet;

pub mod l_mood_release_group;
pub use l_mood_release_group::LMoodReleaseGroup;
pub mod l_mood_release_group_db_set;
pub use l_mood_release_group_db_set::LMoodReleaseGroupSet;

pub mod l_mood_series;
pub use l_mood_series::LMoodSeries;
pub mod l_mood_series_db_set;
pub use l_mood_series_db_set::LMoodSeriesSet;

pub mod l_mood_url;
pub use l_mood_url::LMoodUrl;
pub mod l_mood_url_db_set;
pub use l_mood_url_db_set::LMoodUrlSet;

pub mod l_mood_work;
pub use l_mood_work::LMoodWork;
pub mod l_mood_work_db_set;
pub use l_mood_work_db_set::LMoodWorkSet;

pub mod l_place_place;
pub use l_place_place::LPlacePlace;
pub mod l_place_place_db_set;
pub use l_place_place_db_set::LPlacePlaceSet;

pub mod l_place_recording;
pub use l_place_recording::LPlaceRecording;
pub mod l_place_recording_db_set;
pub use l_place_recording_db_set::LPlaceRecordingSet;

pub mod l_place_release;
pub use l_place_release::LPlaceRelease;
pub mod l_place_release_db_set;
pub use l_place_release_db_set::LPlaceReleaseSet;

pub mod l_place_release_group;
pub use l_place_release_group::LPlaceReleaseGroup;
pub mod l_place_release_group_db_set;
pub use l_place_release_group_db_set::LPlaceReleaseGroupSet;

pub mod l_place_series;
pub use l_place_series::LPlaceSeries;
pub mod l_place_series_db_set;
pub use l_place_series_db_set::LPlaceSeriesSet;

pub mod l_place_url;
pub use l_place_url::LPlaceUrl;
pub mod l_place_url_db_set;
pub use l_place_url_db_set::LPlaceUrlSet;

pub mod l_place_work;
pub use l_place_work::LPlaceWork;
pub mod l_place_work_db_set;
pub use l_place_work_db_set::LPlaceWorkSet;

pub mod l_recording_recording;
pub use l_recording_recording::LRecordingRecording;
pub mod l_recording_recording_db_set;
pub use l_recording_recording_db_set::LRecordingRecordingSet;

pub mod l_recording_release;
pub use l_recording_release::LRecordingRelease;
pub mod l_recording_release_db_set;
pub use l_recording_release_db_set::LRecordingReleaseSet;

pub mod l_recording_release_group;
pub use l_recording_release_group::LRecordingReleaseGroup;
pub mod l_recording_release_group_db_set;
pub use l_recording_release_group_db_set::LRecordingReleaseGroupSet;

pub mod l_recording_series;
pub use l_recording_series::LRecordingSeries;
pub mod l_recording_series_db_set;
pub use l_recording_series_db_set::LRecordingSeriesSet;

pub mod l_recording_url;
pub use l_recording_url::LRecordingUrl;
pub mod l_recording_url_db_set;
pub use l_recording_url_db_set::LRecordingUrlSet;

pub mod l_recording_work;
pub use l_recording_work::LRecordingWork;
pub mod l_recording_work_db_set;
pub use l_recording_work_db_set::LRecordingWorkSet;

pub mod l_release_group_release_group;
pub use l_release_group_release_group::LReleaseGroupReleaseGroup;
pub mod l_release_group_release_group_db_set;
pub use l_release_group_release_group_db_set::LReleaseGroupReleaseGroupSet;

pub mod l_release_group_series;
pub use l_release_group_series::LReleaseGroupSeries;
pub mod l_release_group_series_db_set;
pub use l_release_group_series_db_set::LReleaseGroupSeriesSet;

pub mod l_release_group_url;
pub use l_release_group_url::LReleaseGroupUrl;
pub mod l_release_group_url_db_set;
pub use l_release_group_url_db_set::LReleaseGroupUrlSet;

pub mod l_release_group_work;
pub use l_release_group_work::LReleaseGroupWork;
pub mod l_release_group_work_db_set;
pub use l_release_group_work_db_set::LReleaseGroupWorkSet;

pub mod l_release_release;
pub use l_release_release::LReleaseRelease;
pub mod l_release_release_db_set;
pub use l_release_release_db_set::LReleaseReleaseSet;

pub mod l_release_release_group;
pub use l_release_release_group::LReleaseReleaseGroup;
pub mod l_release_release_group_db_set;
pub use l_release_release_group_db_set::LReleaseReleaseGroupSet;

pub mod l_release_series;
pub use l_release_series::LReleaseSeries;
pub mod l_release_series_db_set;
pub use l_release_series_db_set::LReleaseSeriesSet;

pub mod l_release_url;
pub use l_release_url::LReleaseUrl;
pub mod l_release_url_db_set;
pub use l_release_url_db_set::LReleaseUrlSet;

pub mod l_release_work;
pub use l_release_work::LReleaseWork;
pub mod l_release_work_db_set;
pub use l_release_work_db_set::LReleaseWorkSet;

pub mod l_series_series;
pub use l_series_series::LSeriesSeries;
pub mod l_series_series_db_set;
pub use l_series_series_db_set::LSeriesSeriesSet;

pub mod l_series_url;
pub use l_series_url::LSeriesUrl;
pub mod l_series_url_db_set;
pub use l_series_url_db_set::LSeriesUrlSet;

pub mod l_series_work;
pub use l_series_work::LSeriesWork;
pub mod l_series_work_db_set;
pub use l_series_work_db_set::LSeriesWorkSet;

pub mod l_url_url;
pub use l_url_url::LUrlUrl;
pub mod l_url_url_db_set;
pub use l_url_url_db_set::LUrlUrlSet;

pub mod l_url_work;
pub use l_url_work::LUrlWork;
pub mod l_url_work_db_set;
pub use l_url_work_db_set::LUrlWorkSet;

pub mod l_work_work;
pub use l_work_work::LWorkWork;
pub mod l_work_work_db_set;
pub use l_work_work_db_set::LWorkWorkSet;

pub mod label;
pub use label::Label;
pub mod label_db_set;
pub use label_db_set::LabelSet;

pub mod label_alias;
pub use label_alias::LabelAlias;
pub mod label_alias_db_set;
pub use label_alias_db_set::LabelAliasSet;

pub mod label_alias_type;
pub use label_alias_type::LabelAliasType;
pub mod label_alias_type_db_set;
pub use label_alias_type_db_set::LabelAliasTypeSet;

pub mod label_annotation;
pub use label_annotation::LabelAnnotation;
pub mod label_annotation_db_set;
pub use label_annotation_db_set::LabelAnnotationSet;

pub mod label_attribute;
pub use label_attribute::LabelAttribute;
pub mod label_attribute_db_set;
pub use label_attribute_db_set::LabelAttributeSet;

pub mod label_attribute_type;
pub use label_attribute_type::LabelAttributeType;
pub mod label_attribute_type_db_set;
pub use label_attribute_type_db_set::LabelAttributeTypeSet;

pub mod label_attribute_type_allowed_value;
pub use label_attribute_type_allowed_value::LabelAttributeTypeAllowedValue;
pub mod label_attribute_type_allowed_value_db_set;
pub use label_attribute_type_allowed_value_db_set::LabelAttributeTypeAllowedValueSet;

pub mod label_gid_redirect;
pub use label_gid_redirect::LabelGidRedirect;
pub mod label_gid_redirect_db_set;
pub use label_gid_redirect_db_set::LabelGidRedirectSet;

pub mod label_ipi;
pub use label_ipi::LabelIpi;
pub mod label_ipi_db_set;
pub use label_ipi_db_set::LabelIpiSet;

pub mod label_isni;
pub use label_isni::LabelIsni;
pub mod label_isni_db_set;
pub use label_isni_db_set::LabelIsniSet;

pub mod label_meta;
pub use label_meta::LabelMeta;
pub mod label_meta_db_set;
pub use label_meta_db_set::LabelMetaSet;

pub mod label_rating_raw;
pub use label_rating_raw::LabelRatingRaw;
pub mod label_rating_raw_db_set;
pub use label_rating_raw_db_set::LabelRatingRawSet;

pub mod label_tag;
pub use label_tag::LabelTag;
pub mod label_tag_db_set;
pub use label_tag_db_set::LabelTagSet;

pub mod label_tag_raw;
pub use label_tag_raw::LabelTagRaw;
pub mod label_tag_raw_db_set;
pub use label_tag_raw_db_set::LabelTagRawSet;

pub mod label_type;
pub use label_type::LabelType;
pub mod label_type_db_set;
pub use label_type_db_set::LabelTypeSet;

pub mod language;
pub use language::Language;
pub mod language_db_set;
pub use language_db_set::LanguageSet;

pub mod link;
pub use link::Link;
pub mod link_db_set;
pub use link_db_set::LinkSet;

pub mod link_attribute;
pub use link_attribute::LinkAttribute;
pub mod link_attribute_db_set;
pub use link_attribute_db_set::LinkAttributeSet;

pub mod link_attribute_credit;
pub use link_attribute_credit::LinkAttributeCredit;
pub mod link_attribute_credit_db_set;
pub use link_attribute_credit_db_set::LinkAttributeCreditSet;

pub mod link_attribute_text_value;
pub use link_attribute_text_value::LinkAttributeTextValue;
pub mod link_attribute_text_value_db_set;
pub use link_attribute_text_value_db_set::LinkAttributeTextValueSet;

pub mod link_attribute_type;
pub use link_attribute_type::LinkAttributeType;
pub mod link_attribute_type_db_set;
pub use link_attribute_type_db_set::LinkAttributeTypeSet;

pub mod link_creditable_attribute_type;
pub use link_creditable_attribute_type::LinkCreditableAttributeType;
pub mod link_creditable_attribute_type_db_set;
pub use link_creditable_attribute_type_db_set::LinkCreditableAttributeTypeSet;

pub mod link_text_attribute_type;
pub use link_text_attribute_type::LinkTextAttributeType;
pub mod link_text_attribute_type_db_set;
pub use link_text_attribute_type_db_set::LinkTextAttributeTypeSet;

pub mod link_type;
pub use link_type::LinkType;
pub mod link_type_db_set;
pub use link_type_db_set::LinkTypeSet;

pub mod link_type_attribute_type;
pub use link_type_attribute_type::LinkTypeAttributeType;
pub mod link_type_attribute_type_db_set;
pub use link_type_attribute_type_db_set::LinkTypeAttributeTypeSet;

pub mod medium;
pub use medium::Medium;
pub mod medium_db_set;
pub use medium_db_set::MediumSet;

pub mod medium_attribute;
pub use medium_attribute::MediumAttribute;
pub mod medium_attribute_db_set;
pub use medium_attribute_db_set::MediumAttributeSet;

pub mod medium_attribute_type;
pub use medium_attribute_type::MediumAttributeType;
pub mod medium_attribute_type_db_set;
pub use medium_attribute_type_db_set::MediumAttributeTypeSet;

pub mod medium_attribute_type_allowed_format;
pub use medium_attribute_type_allowed_format::MediumAttributeTypeAllowedFormat;
pub mod medium_attribute_type_allowed_format_db_set;
pub use medium_attribute_type_allowed_format_db_set::MediumAttributeTypeAllowedFormatSet;

pub mod medium_attribute_type_allowed_value;
pub use medium_attribute_type_allowed_value::MediumAttributeTypeAllowedValue;
pub mod medium_attribute_type_allowed_value_db_set;
pub use medium_attribute_type_allowed_value_db_set::MediumAttributeTypeAllowedValueSet;

pub mod medium_attribute_type_allowed_value_allowed_format;
pub use medium_attribute_type_allowed_value_allowed_format::MediumAttributeTypeAllowedValueAllowedFormat;
pub mod medium_attribute_type_allowed_value_allowed_format_db_set;
pub use medium_attribute_type_allowed_value_allowed_format_db_set::MediumAttributeTypeAllowedValueAllowedFormatSet;

pub mod medium_cdtoc;
pub use medium_cdtoc::MediumCdtoc;
pub mod medium_cdtoc_db_set;
pub use medium_cdtoc_db_set::MediumCdtocSet;

pub mod medium_format;
pub use medium_format::MediumFormat;
pub mod medium_format_db_set;
pub use medium_format_db_set::MediumFormatSet;

pub mod medium_index;
pub use medium_index::MediumIndex;
pub mod medium_index_db_set;
pub use medium_index_db_set::MediumIndexSet;

pub mod medium_track_durations;
pub use medium_track_durations::MediumTrackDurations;
pub mod medium_track_durations_db_set;
pub use medium_track_durations_db_set::MediumTrackDurationsSet;

pub mod mood;
pub use mood::Mood;
pub mod mood_db_set;
pub use mood_db_set::MoodSet;

pub mod mood_alias;
pub use mood_alias::MoodAlias;
pub mod mood_alias_db_set;
pub use mood_alias_db_set::MoodAliasSet;

pub mod mood_alias_type;
pub use mood_alias_type::MoodAliasType;
pub mod mood_alias_type_db_set;
pub use mood_alias_type_db_set::MoodAliasTypeSet;

pub mod mood_annotation;
pub use mood_annotation::MoodAnnotation;
pub mod mood_annotation_db_set;
pub use mood_annotation_db_set::MoodAnnotationSet;

pub mod old_editor_name;
pub use old_editor_name::OldEditorName;
pub mod old_editor_name_db_set;
pub use old_editor_name_db_set::OldEditorNameSet;

pub mod orderable_link_type;
pub use orderable_link_type::OrderableLinkType;
pub mod orderable_link_type_db_set;
pub use orderable_link_type_db_set::OrderableLinkTypeSet;

pub mod place;
pub use place::Place;
pub mod place_db_set;
pub use place_db_set::PlaceSet;

pub mod place_alias;
pub use place_alias::PlaceAlias;
pub mod place_alias_db_set;
pub use place_alias_db_set::PlaceAliasSet;

pub mod place_alias_type;
pub use place_alias_type::PlaceAliasType;
pub mod place_alias_type_db_set;
pub use place_alias_type_db_set::PlaceAliasTypeSet;

pub mod place_annotation;
pub use place_annotation::PlaceAnnotation;
pub mod place_annotation_db_set;
pub use place_annotation_db_set::PlaceAnnotationSet;

pub mod place_attribute;
pub use place_attribute::PlaceAttribute;
pub mod place_attribute_db_set;
pub use place_attribute_db_set::PlaceAttributeSet;

pub mod place_attribute_type;
pub use place_attribute_type::PlaceAttributeType;
pub mod place_attribute_type_db_set;
pub use place_attribute_type_db_set::PlaceAttributeTypeSet;

pub mod place_attribute_type_allowed_value;
pub use place_attribute_type_allowed_value::PlaceAttributeTypeAllowedValue;
pub mod place_attribute_type_allowed_value_db_set;
pub use place_attribute_type_allowed_value_db_set::PlaceAttributeTypeAllowedValueSet;

pub mod place_gid_redirect;
pub use place_gid_redirect::PlaceGidRedirect;
pub mod place_gid_redirect_db_set;
pub use place_gid_redirect_db_set::PlaceGidRedirectSet;

pub mod place_meta;
pub use place_meta::PlaceMeta;
pub mod place_meta_db_set;
pub use place_meta_db_set::PlaceMetaSet;

pub mod place_rating_raw;
pub use place_rating_raw::PlaceRatingRaw;
pub mod place_rating_raw_db_set;
pub use place_rating_raw_db_set::PlaceRatingRawSet;

pub mod place_tag;
pub use place_tag::PlaceTag;
pub mod place_tag_db_set;
pub use place_tag_db_set::PlaceTagSet;

pub mod place_tag_raw;
pub use place_tag_raw::PlaceTagRaw;
pub mod place_tag_raw_db_set;
pub use place_tag_raw_db_set::PlaceTagRawSet;

pub mod place_type;
pub use place_type::PlaceType;
pub mod place_type_db_set;
pub use place_type_db_set::PlaceTypeSet;

pub mod recording;
pub use recording::Recording;
pub mod recording_db_set;
pub use recording_db_set::RecordingSet;

pub mod recording_alias;
pub use recording_alias::RecordingAlias;
pub mod recording_alias_db_set;
pub use recording_alias_db_set::RecordingAliasSet;

pub mod recording_alias_type;
pub use recording_alias_type::RecordingAliasType;
pub mod recording_alias_type_db_set;
pub use recording_alias_type_db_set::RecordingAliasTypeSet;

pub mod recording_annotation;
pub use recording_annotation::RecordingAnnotation;
pub mod recording_annotation_db_set;
pub use recording_annotation_db_set::RecordingAnnotationSet;

pub mod recording_attribute;
pub use recording_attribute::RecordingAttribute;
pub mod recording_attribute_db_set;
pub use recording_attribute_db_set::RecordingAttributeSet;

pub mod recording_attribute_type;
pub use recording_attribute_type::RecordingAttributeType;
pub mod recording_attribute_type_db_set;
pub use recording_attribute_type_db_set::RecordingAttributeTypeSet;

pub mod recording_attribute_type_allowed_value;
pub use recording_attribute_type_allowed_value::RecordingAttributeTypeAllowedValue;
pub mod recording_attribute_type_allowed_value_db_set;
pub use recording_attribute_type_allowed_value_db_set::RecordingAttributeTypeAllowedValueSet;

pub mod recording_first_release_date;
pub use recording_first_release_date::RecordingFirstReleaseDate;
pub mod recording_first_release_date_db_set;
pub use recording_first_release_date_db_set::RecordingFirstReleaseDateSet;

pub mod recording_gid_redirect;
pub use recording_gid_redirect::RecordingGidRedirect;
pub mod recording_gid_redirect_db_set;
pub use recording_gid_redirect_db_set::RecordingGidRedirectSet;

pub mod recording_meta;
pub use recording_meta::RecordingMeta;
pub mod recording_meta_db_set;
pub use recording_meta_db_set::RecordingMetaSet;

pub mod recording_rating_raw;
pub use recording_rating_raw::RecordingRatingRaw;
pub mod recording_rating_raw_db_set;
pub use recording_rating_raw_db_set::RecordingRatingRawSet;

pub mod recording_series;
pub use recording_series::RecordingSeries;
pub mod recording_series_db_set;
pub use recording_series_db_set::RecordingSeriesSet;

pub mod recording_tag;
pub use recording_tag::RecordingTag;
pub mod recording_tag_db_set;
pub use recording_tag_db_set::RecordingTagSet;

pub mod recording_tag_raw;
pub use recording_tag_raw::RecordingTagRaw;
pub mod recording_tag_raw_db_set;
pub use recording_tag_raw_db_set::RecordingTagRawSet;

pub mod release;
pub use release::Release;
pub mod release_db_set;
pub use release_db_set::ReleaseSet;

pub mod release_alias;
pub use release_alias::ReleaseAlias;
pub mod release_alias_db_set;
pub use release_alias_db_set::ReleaseAliasSet;

pub mod release_alias_type;
pub use release_alias_type::ReleaseAliasType;
pub mod release_alias_type_db_set;
pub use release_alias_type_db_set::ReleaseAliasTypeSet;

pub mod release_annotation;
pub use release_annotation::ReleaseAnnotation;
pub mod release_annotation_db_set;
pub use release_annotation_db_set::ReleaseAnnotationSet;

pub mod release_attribute;
pub use release_attribute::ReleaseAttribute;
pub mod release_attribute_db_set;
pub use release_attribute_db_set::ReleaseAttributeSet;

pub mod release_attribute_type;
pub use release_attribute_type::ReleaseAttributeType;
pub mod release_attribute_type_db_set;
pub use release_attribute_type_db_set::ReleaseAttributeTypeSet;

pub mod release_attribute_type_allowed_value;
pub use release_attribute_type_allowed_value::ReleaseAttributeTypeAllowedValue;
pub mod release_attribute_type_allowed_value_db_set;
pub use release_attribute_type_allowed_value_db_set::ReleaseAttributeTypeAllowedValueSet;

pub mod release_country;
pub use release_country::ReleaseCountry;
pub mod release_country_db_set;
pub use release_country_db_set::ReleaseCountrySet;

pub mod release_event;
pub use release_event::ReleaseEvent;
pub mod release_event_db_set;
pub use release_event_db_set::ReleaseEventSet;

pub mod release_first_release_date;
pub use release_first_release_date::ReleaseFirstReleaseDate;
pub mod release_first_release_date_db_set;
pub use release_first_release_date_db_set::ReleaseFirstReleaseDateSet;

pub mod release_gid_redirect;
pub use release_gid_redirect::ReleaseGidRedirect;
pub mod release_gid_redirect_db_set;
pub use release_gid_redirect_db_set::ReleaseGidRedirectSet;

pub mod release_group;
pub use release_group::ReleaseGroup;
pub mod release_group_db_set;
pub use release_group_db_set::ReleaseGroupSet;

pub mod release_group_alias;
pub use release_group_alias::ReleaseGroupAlias;
pub mod release_group_alias_db_set;
pub use release_group_alias_db_set::ReleaseGroupAliasSet;

pub mod release_group_alias_type;
pub use release_group_alias_type::ReleaseGroupAliasType;
pub mod release_group_alias_type_db_set;
pub use release_group_alias_type_db_set::ReleaseGroupAliasTypeSet;

pub mod release_group_annotation;
pub use release_group_annotation::ReleaseGroupAnnotation;
pub mod release_group_annotation_db_set;
pub use release_group_annotation_db_set::ReleaseGroupAnnotationSet;

pub mod release_group_attribute;
pub use release_group_attribute::ReleaseGroupAttribute;
pub mod release_group_attribute_db_set;
pub use release_group_attribute_db_set::ReleaseGroupAttributeSet;

pub mod release_group_attribute_type;
pub use release_group_attribute_type::ReleaseGroupAttributeType;
pub mod release_group_attribute_type_db_set;
pub use release_group_attribute_type_db_set::ReleaseGroupAttributeTypeSet;

pub mod release_group_attribute_type_allowed_value;
pub use release_group_attribute_type_allowed_value::ReleaseGroupAttributeTypeAllowedValue;
pub mod release_group_attribute_type_allowed_value_db_set;
pub use release_group_attribute_type_allowed_value_db_set::ReleaseGroupAttributeTypeAllowedValueSet;

pub mod release_group_gid_redirect;
pub use release_group_gid_redirect::ReleaseGroupGidRedirect;
pub mod release_group_gid_redirect_db_set;
pub use release_group_gid_redirect_db_set::ReleaseGroupGidRedirectSet;

pub mod release_group_meta;
pub use release_group_meta::ReleaseGroupMeta;
pub mod release_group_meta_db_set;
pub use release_group_meta_db_set::ReleaseGroupMetaSet;

pub mod release_group_primary_type;
pub use release_group_primary_type::ReleaseGroupPrimaryType;
pub mod release_group_primary_type_db_set;
pub use release_group_primary_type_db_set::ReleaseGroupPrimaryTypeSet;

pub mod release_group_rating_raw;
pub use release_group_rating_raw::ReleaseGroupRatingRaw;
pub mod release_group_rating_raw_db_set;
pub use release_group_rating_raw_db_set::ReleaseGroupRatingRawSet;

pub mod release_group_secondary_type;
pub use release_group_secondary_type::ReleaseGroupSecondaryType;
pub mod release_group_secondary_type_db_set;
pub use release_group_secondary_type_db_set::ReleaseGroupSecondaryTypeSet;

pub mod release_group_secondary_type_join;
pub use release_group_secondary_type_join::ReleaseGroupSecondaryTypeJoin;
pub mod release_group_secondary_type_join_db_set;
pub use release_group_secondary_type_join_db_set::ReleaseGroupSecondaryTypeJoinSet;

pub mod release_group_series;
pub use release_group_series::ReleaseGroupSeries;
pub mod release_group_series_db_set;
pub use release_group_series_db_set::ReleaseGroupSeriesSet;

pub mod release_group_tag;
pub use release_group_tag::ReleaseGroupTag;
pub mod release_group_tag_db_set;
pub use release_group_tag_db_set::ReleaseGroupTagSet;

pub mod release_group_tag_raw;
pub use release_group_tag_raw::ReleaseGroupTagRaw;
pub mod release_group_tag_raw_db_set;
pub use release_group_tag_raw_db_set::ReleaseGroupTagRawSet;

pub mod release_label;
pub use release_label::ReleaseLabel;
pub mod release_label_db_set;
pub use release_label_db_set::ReleaseLabelSet;

pub mod release_meta;
pub use release_meta::ReleaseMeta;
pub mod release_meta_db_set;
pub use release_meta_db_set::ReleaseMetaSet;

pub mod release_packaging;
pub use release_packaging::ReleasePackaging;
pub mod release_packaging_db_set;
pub use release_packaging_db_set::ReleasePackagingSet;

pub mod release_raw;
pub use release_raw::ReleaseRaw;
pub mod release_raw_db_set;
pub use release_raw_db_set::ReleaseRawSet;

pub mod release_series;
pub use release_series::ReleaseSeries;
pub mod release_series_db_set;
pub use release_series_db_set::ReleaseSeriesSet;

pub mod release_status;
pub use release_status::ReleaseStatus;
pub mod release_status_db_set;
pub use release_status_db_set::ReleaseStatusSet;

pub mod release_tag;
pub use release_tag::ReleaseTag;
pub mod release_tag_db_set;
pub use release_tag_db_set::ReleaseTagSet;

pub mod release_tag_raw;
pub use release_tag_raw::ReleaseTagRaw;
pub mod release_tag_raw_db_set;
pub use release_tag_raw_db_set::ReleaseTagRawSet;

pub mod release_unknown_country;
pub use release_unknown_country::ReleaseUnknownCountry;
pub mod release_unknown_country_db_set;
pub use release_unknown_country_db_set::ReleaseUnknownCountrySet;

pub mod replication_control;
pub use replication_control::ReplicationControl;
pub mod replication_control_db_set;
pub use replication_control_db_set::ReplicationControlSet;

pub mod script;
pub use script::Script;
pub mod script_db_set;
pub use script_db_set::ScriptSet;

pub mod series;
pub use series::Series;
pub mod series_db_set;
pub use series_db_set::SeriesSet;

pub mod series_alias;
pub use series_alias::SeriesAlias;
pub mod series_alias_db_set;
pub use series_alias_db_set::SeriesAliasSet;

pub mod series_alias_type;
pub use series_alias_type::SeriesAliasType;
pub mod series_alias_type_db_set;
pub use series_alias_type_db_set::SeriesAliasTypeSet;

pub mod series_annotation;
pub use series_annotation::SeriesAnnotation;
pub mod series_annotation_db_set;
pub use series_annotation_db_set::SeriesAnnotationSet;

pub mod series_attribute;
pub use series_attribute::SeriesAttribute;
pub mod series_attribute_db_set;
pub use series_attribute_db_set::SeriesAttributeSet;

pub mod series_attribute_type;
pub use series_attribute_type::SeriesAttributeType;
pub mod series_attribute_type_db_set;
pub use series_attribute_type_db_set::SeriesAttributeTypeSet;

pub mod series_attribute_type_allowed_value;
pub use series_attribute_type_allowed_value::SeriesAttributeTypeAllowedValue;
pub mod series_attribute_type_allowed_value_db_set;
pub use series_attribute_type_allowed_value_db_set::SeriesAttributeTypeAllowedValueSet;

pub mod series_gid_redirect;
pub use series_gid_redirect::SeriesGidRedirect;
pub mod series_gid_redirect_db_set;
pub use series_gid_redirect_db_set::SeriesGidRedirectSet;

pub mod series_ordering_type;
pub use series_ordering_type::SeriesOrderingType;
pub mod series_ordering_type_db_set;
pub use series_ordering_type_db_set::SeriesOrderingTypeSet;

pub mod series_tag;
pub use series_tag::SeriesTag;
pub mod series_tag_db_set;
pub use series_tag_db_set::SeriesTagSet;

pub mod series_tag_raw;
pub use series_tag_raw::SeriesTagRaw;
pub mod series_tag_raw_db_set;
pub use series_tag_raw_db_set::SeriesTagRawSet;

pub mod series_type;
pub use series_type::SeriesType;
pub mod series_type_db_set;
pub use series_type_db_set::SeriesTypeSet;

pub mod tag;
pub use tag::Tag;
pub mod tag_db_set;
pub use tag_db_set::TagSet;

pub mod tag_relation;
pub use tag_relation::TagRelation;
pub mod tag_relation_db_set;
pub use tag_relation_db_set::TagRelationSet;

pub mod track;
pub use track::Track;
pub mod track_db_set;
pub use track_db_set::TrackSet;

pub mod track_gid_redirect;
pub use track_gid_redirect::TrackGidRedirect;
pub mod track_gid_redirect_db_set;
pub use track_gid_redirect_db_set::TrackGidRedirectSet;

pub mod track_raw;
pub use track_raw::TrackRaw;
pub mod track_raw_db_set;
pub use track_raw_db_set::TrackRawSet;

pub mod unreferenced_row_log;
pub use unreferenced_row_log::UnreferencedRowLog;
pub mod unreferenced_row_log_db_set;
pub use unreferenced_row_log_db_set::UnreferencedRowLogSet;

pub mod url;
pub use url::Url;
pub mod url_db_set;
pub use url_db_set::UrlSet;

pub mod url_gid_redirect;
pub use url_gid_redirect::UrlGidRedirect;
pub mod url_gid_redirect_db_set;
pub use url_gid_redirect_db_set::UrlGidRedirectSet;

pub mod vote;
pub use vote::Vote;
pub mod vote_db_set;
pub use vote_db_set::VoteSet;

pub mod work;
pub use work::Work;
pub mod work_db_set;
pub use work_db_set::WorkSet;

pub mod work_alias;
pub use work_alias::WorkAlias;
pub mod work_alias_db_set;
pub use work_alias_db_set::WorkAliasSet;

pub mod work_alias_type;
pub use work_alias_type::WorkAliasType;
pub mod work_alias_type_db_set;
pub use work_alias_type_db_set::WorkAliasTypeSet;

pub mod work_annotation;
pub use work_annotation::WorkAnnotation;
pub mod work_annotation_db_set;
pub use work_annotation_db_set::WorkAnnotationSet;

pub mod work_attribute;
pub use work_attribute::WorkAttribute;
pub mod work_attribute_db_set;
pub use work_attribute_db_set::WorkAttributeSet;

pub mod work_attribute_type;
pub use work_attribute_type::WorkAttributeType;
pub mod work_attribute_type_db_set;
pub use work_attribute_type_db_set::WorkAttributeTypeSet;

pub mod work_attribute_type_allowed_value;
pub use work_attribute_type_allowed_value::WorkAttributeTypeAllowedValue;
pub mod work_attribute_type_allowed_value_db_set;
pub use work_attribute_type_allowed_value_db_set::WorkAttributeTypeAllowedValueSet;

pub mod work_gid_redirect;
pub use work_gid_redirect::WorkGidRedirect;
pub mod work_gid_redirect_db_set;
pub use work_gid_redirect_db_set::WorkGidRedirectSet;

pub mod work_language;
pub use work_language::WorkLanguage;
pub mod work_language_db_set;
pub use work_language_db_set::WorkLanguageSet;

pub mod work_meta;
pub use work_meta::WorkMeta;
pub mod work_meta_db_set;
pub use work_meta_db_set::WorkMetaSet;

pub mod work_rating_raw;
pub use work_rating_raw::WorkRatingRaw;
pub mod work_rating_raw_db_set;
pub use work_rating_raw_db_set::WorkRatingRawSet;

pub mod work_series;
pub use work_series::WorkSeries;
pub mod work_series_db_set;
pub use work_series_db_set::WorkSeriesSet;

pub mod work_tag;
pub use work_tag::WorkTag;
pub mod work_tag_db_set;
pub use work_tag_db_set::WorkTagSet;

pub mod work_tag_raw;
pub use work_tag_raw::WorkTagRaw;
pub mod work_tag_raw_db_set;
pub use work_tag_raw_db_set::WorkTagRawSet;

pub mod work_type;
pub use work_type::WorkType;
pub mod work_type_db_set;
pub use work_type_db_set::WorkTypeSet;


pub struct MusicbrainzDbContext;

impl MusicbrainzDbContext {
  pub fn alternative_medium(&self) -> AlternativeMediumSet { AlternativeMediumSet }

  pub fn alternative_medium_track(&self) -> AlternativeMediumTrackSet { AlternativeMediumTrackSet }

  pub fn alternative_release(&self) -> AlternativeReleaseSet { AlternativeReleaseSet }

  pub fn alternative_release_type(&self) -> AlternativeReleaseTypeSet { AlternativeReleaseTypeSet }

  pub fn alternative_track(&self) -> AlternativeTrackSet { AlternativeTrackSet }

  pub fn annotation(&self) -> AnnotationSet { AnnotationSet }

  pub fn application(&self) -> ApplicationSet { ApplicationSet }

  pub fn area(&self) -> AreaSet { AreaSet }

  pub fn area_alias(&self) -> AreaAliasSet { AreaAliasSet }

  pub fn area_alias_type(&self) -> AreaAliasTypeSet { AreaAliasTypeSet }

  pub fn area_annotation(&self) -> AreaAnnotationSet { AreaAnnotationSet }

  pub fn area_attribute(&self) -> AreaAttributeSet { AreaAttributeSet }

  pub fn area_attribute_type(&self) -> AreaAttributeTypeSet { AreaAttributeTypeSet }

  pub fn area_attribute_type_allowed_value(&self) -> AreaAttributeTypeAllowedValueSet { AreaAttributeTypeAllowedValueSet }

  pub fn area_containment(&self) -> AreaContainmentSet { AreaContainmentSet }

  pub fn area_gid_redirect(&self) -> AreaGidRedirectSet { AreaGidRedirectSet }

  pub fn area_tag(&self) -> AreaTagSet { AreaTagSet }

  pub fn area_tag_raw(&self) -> AreaTagRawSet { AreaTagRawSet }

  pub fn area_type(&self) -> AreaTypeSet { AreaTypeSet }

  pub fn artist(&self) -> ArtistSet { ArtistSet }

  pub fn artist_alias(&self) -> ArtistAliasSet { ArtistAliasSet }

  pub fn artist_alias_type(&self) -> ArtistAliasTypeSet { ArtistAliasTypeSet }

  pub fn artist_annotation(&self) -> ArtistAnnotationSet { ArtistAnnotationSet }

  pub fn artist_attribute(&self) -> ArtistAttributeSet { ArtistAttributeSet }

  pub fn artist_attribute_type(&self) -> ArtistAttributeTypeSet { ArtistAttributeTypeSet }

  pub fn artist_attribute_type_allowed_value(&self) -> ArtistAttributeTypeAllowedValueSet { ArtistAttributeTypeAllowedValueSet }

  pub fn artist_credit(&self) -> ArtistCreditSet { ArtistCreditSet }

  pub fn artist_credit_gid_redirect(&self) -> ArtistCreditGidRedirectSet { ArtistCreditGidRedirectSet }

  pub fn artist_credit_name(&self) -> ArtistCreditNameSet { ArtistCreditNameSet }

  pub fn artist_gid_redirect(&self) -> ArtistGidRedirectSet { ArtistGidRedirectSet }

  pub fn artist_ipi(&self) -> ArtistIpiSet { ArtistIpiSet }

  pub fn artist_isni(&self) -> ArtistIsniSet { ArtistIsniSet }

  pub fn artist_meta(&self) -> ArtistMetaSet { ArtistMetaSet }

  pub fn artist_rating_raw(&self) -> ArtistRatingRawSet { ArtistRatingRawSet }

  pub fn artist_release(&self) -> ArtistReleaseSet { ArtistReleaseSet }

  pub fn artist_release_group(&self) -> ArtistReleaseGroupSet { ArtistReleaseGroupSet }

  pub fn artist_release_group_nonva(&self) -> ArtistReleaseGroupNonvaSet { ArtistReleaseGroupNonvaSet }

  pub fn artist_release_group_pending_update(&self) -> ArtistReleaseGroupPendingUpdateSet { ArtistReleaseGroupPendingUpdateSet }

  pub fn artist_release_group_va(&self) -> ArtistReleaseGroupVaSet { ArtistReleaseGroupVaSet }

  pub fn artist_release_nonva(&self) -> ArtistReleaseNonvaSet { ArtistReleaseNonvaSet }

  pub fn artist_release_pending_update(&self) -> ArtistReleasePendingUpdateSet { ArtistReleasePendingUpdateSet }

  pub fn artist_release_va(&self) -> ArtistReleaseVaSet { ArtistReleaseVaSet }

  pub fn artist_series(&self) -> ArtistSeriesSet { ArtistSeriesSet }

  pub fn artist_tag(&self) -> ArtistTagSet { ArtistTagSet }

  pub fn artist_tag_raw(&self) -> ArtistTagRawSet { ArtistTagRawSet }

  pub fn artist_type(&self) -> ArtistTypeSet { ArtistTypeSet }

  pub fn autoeditor_election(&self) -> AutoeditorElectionSet { AutoeditorElectionSet }

  pub fn autoeditor_election_vote(&self) -> AutoeditorElectionVoteSet { AutoeditorElectionVoteSet }

  pub fn cdtoc(&self) -> CdtocSet { CdtocSet }

  pub fn cdtoc_raw(&self) -> CdtocRawSet { CdtocRawSet }

  pub fn country_area(&self) -> CountryAreaSet { CountryAreaSet }

  pub fn cube_index(&self) -> CubeIndexSet { CubeIndexSet }

  pub fn deleted_entity(&self) -> DeletedEntitySet { DeletedEntitySet }

  pub fn edit(&self) -> EditSet { EditSet }

  pub fn edit_area(&self) -> EditAreaSet { EditAreaSet }

  pub fn edit_artist(&self) -> EditArtistSet { EditArtistSet }

  pub fn edit_data(&self) -> EditDataSet { EditDataSet }

  pub fn edit_event(&self) -> EditEventSet { EditEventSet }

  pub fn edit_genre(&self) -> EditGenreSet { EditGenreSet }

  pub fn edit_instrument(&self) -> EditInstrumentSet { EditInstrumentSet }

  pub fn edit_label(&self) -> EditLabelSet { EditLabelSet }

  pub fn edit_mood(&self) -> EditMoodSet { EditMoodSet }

  pub fn edit_note(&self) -> EditNoteSet { EditNoteSet }

  pub fn edit_note_change(&self) -> EditNoteChangeSet { EditNoteChangeSet }

  pub fn edit_note_recipient(&self) -> EditNoteRecipientSet { EditNoteRecipientSet }

  pub fn edit_place(&self) -> EditPlaceSet { EditPlaceSet }

  pub fn edit_recording(&self) -> EditRecordingSet { EditRecordingSet }

  pub fn edit_release(&self) -> EditReleaseSet { EditReleaseSet }

  pub fn edit_release_group(&self) -> EditReleaseGroupSet { EditReleaseGroupSet }

  pub fn edit_series(&self) -> EditSeriesSet { EditSeriesSet }

  pub fn edit_url(&self) -> EditUrlSet { EditUrlSet }

  pub fn edit_work(&self) -> EditWorkSet { EditWorkSet }

  pub fn editor(&self) -> EditorSet { EditorSet }

  pub fn editor_collection(&self) -> EditorCollectionSet { EditorCollectionSet }

  pub fn editor_collection_area(&self) -> EditorCollectionAreaSet { EditorCollectionAreaSet }

  pub fn editor_collection_artist(&self) -> EditorCollectionArtistSet { EditorCollectionArtistSet }

  pub fn editor_collection_collaborator(&self) -> EditorCollectionCollaboratorSet { EditorCollectionCollaboratorSet }

  pub fn editor_collection_deleted_entity(&self) -> EditorCollectionDeletedEntitySet { EditorCollectionDeletedEntitySet }

  pub fn editor_collection_event(&self) -> EditorCollectionEventSet { EditorCollectionEventSet }

  pub fn editor_collection_gid_redirect(&self) -> EditorCollectionGidRedirectSet { EditorCollectionGidRedirectSet }

  pub fn editor_collection_instrument(&self) -> EditorCollectionInstrumentSet { EditorCollectionInstrumentSet }

  pub fn editor_collection_label(&self) -> EditorCollectionLabelSet { EditorCollectionLabelSet }

  pub fn editor_collection_place(&self) -> EditorCollectionPlaceSet { EditorCollectionPlaceSet }

  pub fn editor_collection_recording(&self) -> EditorCollectionRecordingSet { EditorCollectionRecordingSet }

  pub fn editor_collection_release(&self) -> EditorCollectionReleaseSet { EditorCollectionReleaseSet }

  pub fn editor_collection_release_group(&self) -> EditorCollectionReleaseGroupSet { EditorCollectionReleaseGroupSet }

  pub fn editor_collection_series(&self) -> EditorCollectionSeriesSet { EditorCollectionSeriesSet }

  pub fn editor_collection_type(&self) -> EditorCollectionTypeSet { EditorCollectionTypeSet }

  pub fn editor_collection_work(&self) -> EditorCollectionWorkSet { EditorCollectionWorkSet }

  pub fn editor_language(&self) -> EditorLanguageSet { EditorLanguageSet }

  pub fn editor_oauth_token(&self) -> EditorOauthTokenSet { EditorOauthTokenSet }

  pub fn editor_preference(&self) -> EditorPreferenceSet { EditorPreferenceSet }

  pub fn editor_subscribe_artist(&self) -> EditorSubscribeArtistSet { EditorSubscribeArtistSet }

  pub fn editor_subscribe_artist_deleted(&self) -> EditorSubscribeArtistDeletedSet { EditorSubscribeArtistDeletedSet }

  pub fn editor_subscribe_collection(&self) -> EditorSubscribeCollectionSet { EditorSubscribeCollectionSet }

  pub fn editor_subscribe_editor(&self) -> EditorSubscribeEditorSet { EditorSubscribeEditorSet }

  pub fn editor_subscribe_label(&self) -> EditorSubscribeLabelSet { EditorSubscribeLabelSet }

  pub fn editor_subscribe_label_deleted(&self) -> EditorSubscribeLabelDeletedSet { EditorSubscribeLabelDeletedSet }

  pub fn editor_subscribe_series(&self) -> EditorSubscribeSeriesSet { EditorSubscribeSeriesSet }

  pub fn editor_subscribe_series_deleted(&self) -> EditorSubscribeSeriesDeletedSet { EditorSubscribeSeriesDeletedSet }

  pub fn event(&self) -> EventSet { EventSet }

  pub fn event_alias(&self) -> EventAliasSet { EventAliasSet }

  pub fn event_alias_type(&self) -> EventAliasTypeSet { EventAliasTypeSet }

  pub fn event_annotation(&self) -> EventAnnotationSet { EventAnnotationSet }

  pub fn event_attribute(&self) -> EventAttributeSet { EventAttributeSet }

  pub fn event_attribute_type(&self) -> EventAttributeTypeSet { EventAttributeTypeSet }

  pub fn event_attribute_type_allowed_value(&self) -> EventAttributeTypeAllowedValueSet { EventAttributeTypeAllowedValueSet }

  pub fn event_gid_redirect(&self) -> EventGidRedirectSet { EventGidRedirectSet }

  pub fn event_meta(&self) -> EventMetaSet { EventMetaSet }

  pub fn event_rating_raw(&self) -> EventRatingRawSet { EventRatingRawSet }

  pub fn event_series(&self) -> EventSeriesSet { EventSeriesSet }

  pub fn event_tag(&self) -> EventTagSet { EventTagSet }

  pub fn event_tag_raw(&self) -> EventTagRawSet { EventTagRawSet }

  pub fn event_type(&self) -> EventTypeSet { EventTypeSet }

  pub fn gender(&self) -> GenderSet { GenderSet }

  pub fn genre(&self) -> GenreSet { GenreSet }

  pub fn genre_alias(&self) -> GenreAliasSet { GenreAliasSet }

  pub fn genre_alias_type(&self) -> GenreAliasTypeSet { GenreAliasTypeSet }

  pub fn genre_annotation(&self) -> GenreAnnotationSet { GenreAnnotationSet }

  pub fn instrument(&self) -> InstrumentSet { InstrumentSet }

  pub fn instrument_alias(&self) -> InstrumentAliasSet { InstrumentAliasSet }

  pub fn instrument_alias_type(&self) -> InstrumentAliasTypeSet { InstrumentAliasTypeSet }

  pub fn instrument_annotation(&self) -> InstrumentAnnotationSet { InstrumentAnnotationSet }

  pub fn instrument_attribute(&self) -> InstrumentAttributeSet { InstrumentAttributeSet }

  pub fn instrument_attribute_type(&self) -> InstrumentAttributeTypeSet { InstrumentAttributeTypeSet }

  pub fn instrument_attribute_type_allowed_value(&self) -> InstrumentAttributeTypeAllowedValueSet { InstrumentAttributeTypeAllowedValueSet }

  pub fn instrument_gid_redirect(&self) -> InstrumentGidRedirectSet { InstrumentGidRedirectSet }

  pub fn instrument_tag(&self) -> InstrumentTagSet { InstrumentTagSet }

  pub fn instrument_tag_raw(&self) -> InstrumentTagRawSet { InstrumentTagRawSet }

  pub fn instrument_type(&self) -> InstrumentTypeSet { InstrumentTypeSet }

  pub fn iso_3166_1(&self) -> Iso31661Set { Iso31661Set }

  pub fn iso_3166_2(&self) -> Iso31662Set { Iso31662Set }

  pub fn iso_3166_3(&self) -> Iso31663Set { Iso31663Set }

  pub fn isrc(&self) -> IsrcSet { IsrcSet }

  pub fn iswc(&self) -> IswcSet { IswcSet }

  pub fn l_area_area(&self) -> LAreaAreaSet { LAreaAreaSet }

  pub fn l_area_artist(&self) -> LAreaArtistSet { LAreaArtistSet }

  pub fn l_area_event(&self) -> LAreaEventSet { LAreaEventSet }

  pub fn l_area_genre(&self) -> LAreaGenreSet { LAreaGenreSet }

  pub fn l_area_instrument(&self) -> LAreaInstrumentSet { LAreaInstrumentSet }

  pub fn l_area_label(&self) -> LAreaLabelSet { LAreaLabelSet }

  pub fn l_area_mood(&self) -> LAreaMoodSet { LAreaMoodSet }

  pub fn l_area_place(&self) -> LAreaPlaceSet { LAreaPlaceSet }

  pub fn l_area_recording(&self) -> LAreaRecordingSet { LAreaRecordingSet }

  pub fn l_area_release(&self) -> LAreaReleaseSet { LAreaReleaseSet }

  pub fn l_area_release_group(&self) -> LAreaReleaseGroupSet { LAreaReleaseGroupSet }

  pub fn l_area_series(&self) -> LAreaSeriesSet { LAreaSeriesSet }

  pub fn l_area_url(&self) -> LAreaUrlSet { LAreaUrlSet }

  pub fn l_area_work(&self) -> LAreaWorkSet { LAreaWorkSet }

  pub fn l_artist_artist(&self) -> LArtistArtistSet { LArtistArtistSet }

  pub fn l_artist_event(&self) -> LArtistEventSet { LArtistEventSet }

  pub fn l_artist_genre(&self) -> LArtistGenreSet { LArtistGenreSet }

  pub fn l_artist_instrument(&self) -> LArtistInstrumentSet { LArtistInstrumentSet }

  pub fn l_artist_label(&self) -> LArtistLabelSet { LArtistLabelSet }

  pub fn l_artist_mood(&self) -> LArtistMoodSet { LArtistMoodSet }

  pub fn l_artist_place(&self) -> LArtistPlaceSet { LArtistPlaceSet }

  pub fn l_artist_recording(&self) -> LArtistRecordingSet { LArtistRecordingSet }

  pub fn l_artist_release(&self) -> LArtistReleaseSet { LArtistReleaseSet }

  pub fn l_artist_release_group(&self) -> LArtistReleaseGroupSet { LArtistReleaseGroupSet }

  pub fn l_artist_series(&self) -> LArtistSeriesSet { LArtistSeriesSet }

  pub fn l_artist_url(&self) -> LArtistUrlSet { LArtistUrlSet }

  pub fn l_artist_work(&self) -> LArtistWorkSet { LArtistWorkSet }

  pub fn l_event_event(&self) -> LEventEventSet { LEventEventSet }

  pub fn l_event_genre(&self) -> LEventGenreSet { LEventGenreSet }

  pub fn l_event_instrument(&self) -> LEventInstrumentSet { LEventInstrumentSet }

  pub fn l_event_label(&self) -> LEventLabelSet { LEventLabelSet }

  pub fn l_event_mood(&self) -> LEventMoodSet { LEventMoodSet }

  pub fn l_event_place(&self) -> LEventPlaceSet { LEventPlaceSet }

  pub fn l_event_recording(&self) -> LEventRecordingSet { LEventRecordingSet }

  pub fn l_event_release(&self) -> LEventReleaseSet { LEventReleaseSet }

  pub fn l_event_release_group(&self) -> LEventReleaseGroupSet { LEventReleaseGroupSet }

  pub fn l_event_series(&self) -> LEventSeriesSet { LEventSeriesSet }

  pub fn l_event_url(&self) -> LEventUrlSet { LEventUrlSet }

  pub fn l_event_work(&self) -> LEventWorkSet { LEventWorkSet }

  pub fn l_genre_genre(&self) -> LGenreGenreSet { LGenreGenreSet }

  pub fn l_genre_instrument(&self) -> LGenreInstrumentSet { LGenreInstrumentSet }

  pub fn l_genre_label(&self) -> LGenreLabelSet { LGenreLabelSet }

  pub fn l_genre_mood(&self) -> LGenreMoodSet { LGenreMoodSet }

  pub fn l_genre_place(&self) -> LGenrePlaceSet { LGenrePlaceSet }

  pub fn l_genre_recording(&self) -> LGenreRecordingSet { LGenreRecordingSet }

  pub fn l_genre_release(&self) -> LGenreReleaseSet { LGenreReleaseSet }

  pub fn l_genre_release_group(&self) -> LGenreReleaseGroupSet { LGenreReleaseGroupSet }

  pub fn l_genre_series(&self) -> LGenreSeriesSet { LGenreSeriesSet }

  pub fn l_genre_url(&self) -> LGenreUrlSet { LGenreUrlSet }

  pub fn l_genre_work(&self) -> LGenreWorkSet { LGenreWorkSet }

  pub fn l_instrument_instrument(&self) -> LInstrumentInstrumentSet { LInstrumentInstrumentSet }

  pub fn l_instrument_label(&self) -> LInstrumentLabelSet { LInstrumentLabelSet }

  pub fn l_instrument_mood(&self) -> LInstrumentMoodSet { LInstrumentMoodSet }

  pub fn l_instrument_place(&self) -> LInstrumentPlaceSet { LInstrumentPlaceSet }

  pub fn l_instrument_recording(&self) -> LInstrumentRecordingSet { LInstrumentRecordingSet }

  pub fn l_instrument_release(&self) -> LInstrumentReleaseSet { LInstrumentReleaseSet }

  pub fn l_instrument_release_group(&self) -> LInstrumentReleaseGroupSet { LInstrumentReleaseGroupSet }

  pub fn l_instrument_series(&self) -> LInstrumentSeriesSet { LInstrumentSeriesSet }

  pub fn l_instrument_url(&self) -> LInstrumentUrlSet { LInstrumentUrlSet }

  pub fn l_instrument_work(&self) -> LInstrumentWorkSet { LInstrumentWorkSet }

  pub fn l_label_label(&self) -> LLabelLabelSet { LLabelLabelSet }

  pub fn l_label_mood(&self) -> LLabelMoodSet { LLabelMoodSet }

  pub fn l_label_place(&self) -> LLabelPlaceSet { LLabelPlaceSet }

  pub fn l_label_recording(&self) -> LLabelRecordingSet { LLabelRecordingSet }

  pub fn l_label_release(&self) -> LLabelReleaseSet { LLabelReleaseSet }

  pub fn l_label_release_group(&self) -> LLabelReleaseGroupSet { LLabelReleaseGroupSet }

  pub fn l_label_series(&self) -> LLabelSeriesSet { LLabelSeriesSet }

  pub fn l_label_url(&self) -> LLabelUrlSet { LLabelUrlSet }

  pub fn l_label_work(&self) -> LLabelWorkSet { LLabelWorkSet }

  pub fn l_mood_mood(&self) -> LMoodMoodSet { LMoodMoodSet }

  pub fn l_mood_place(&self) -> LMoodPlaceSet { LMoodPlaceSet }

  pub fn l_mood_recording(&self) -> LMoodRecordingSet { LMoodRecordingSet }

  pub fn l_mood_release(&self) -> LMoodReleaseSet { LMoodReleaseSet }

  pub fn l_mood_release_group(&self) -> LMoodReleaseGroupSet { LMoodReleaseGroupSet }

  pub fn l_mood_series(&self) -> LMoodSeriesSet { LMoodSeriesSet }

  pub fn l_mood_url(&self) -> LMoodUrlSet { LMoodUrlSet }

  pub fn l_mood_work(&self) -> LMoodWorkSet { LMoodWorkSet }

  pub fn l_place_place(&self) -> LPlacePlaceSet { LPlacePlaceSet }

  pub fn l_place_recording(&self) -> LPlaceRecordingSet { LPlaceRecordingSet }

  pub fn l_place_release(&self) -> LPlaceReleaseSet { LPlaceReleaseSet }

  pub fn l_place_release_group(&self) -> LPlaceReleaseGroupSet { LPlaceReleaseGroupSet }

  pub fn l_place_series(&self) -> LPlaceSeriesSet { LPlaceSeriesSet }

  pub fn l_place_url(&self) -> LPlaceUrlSet { LPlaceUrlSet }

  pub fn l_place_work(&self) -> LPlaceWorkSet { LPlaceWorkSet }

  pub fn l_recording_recording(&self) -> LRecordingRecordingSet { LRecordingRecordingSet }

  pub fn l_recording_release(&self) -> LRecordingReleaseSet { LRecordingReleaseSet }

  pub fn l_recording_release_group(&self) -> LRecordingReleaseGroupSet { LRecordingReleaseGroupSet }

  pub fn l_recording_series(&self) -> LRecordingSeriesSet { LRecordingSeriesSet }

  pub fn l_recording_url(&self) -> LRecordingUrlSet { LRecordingUrlSet }

  pub fn l_recording_work(&self) -> LRecordingWorkSet { LRecordingWorkSet }

  pub fn l_release_group_release_group(&self) -> LReleaseGroupReleaseGroupSet { LReleaseGroupReleaseGroupSet }

  pub fn l_release_group_series(&self) -> LReleaseGroupSeriesSet { LReleaseGroupSeriesSet }

  pub fn l_release_group_url(&self) -> LReleaseGroupUrlSet { LReleaseGroupUrlSet }

  pub fn l_release_group_work(&self) -> LReleaseGroupWorkSet { LReleaseGroupWorkSet }

  pub fn l_release_release(&self) -> LReleaseReleaseSet { LReleaseReleaseSet }

  pub fn l_release_release_group(&self) -> LReleaseReleaseGroupSet { LReleaseReleaseGroupSet }

  pub fn l_release_series(&self) -> LReleaseSeriesSet { LReleaseSeriesSet }

  pub fn l_release_url(&self) -> LReleaseUrlSet { LReleaseUrlSet }

  pub fn l_release_work(&self) -> LReleaseWorkSet { LReleaseWorkSet }

  pub fn l_series_series(&self) -> LSeriesSeriesSet { LSeriesSeriesSet }

  pub fn l_series_url(&self) -> LSeriesUrlSet { LSeriesUrlSet }

  pub fn l_series_work(&self) -> LSeriesWorkSet { LSeriesWorkSet }

  pub fn l_url_url(&self) -> LUrlUrlSet { LUrlUrlSet }

  pub fn l_url_work(&self) -> LUrlWorkSet { LUrlWorkSet }

  pub fn l_work_work(&self) -> LWorkWorkSet { LWorkWorkSet }

  pub fn label(&self) -> LabelSet { LabelSet }

  pub fn label_alias(&self) -> LabelAliasSet { LabelAliasSet }

  pub fn label_alias_type(&self) -> LabelAliasTypeSet { LabelAliasTypeSet }

  pub fn label_annotation(&self) -> LabelAnnotationSet { LabelAnnotationSet }

  pub fn label_attribute(&self) -> LabelAttributeSet { LabelAttributeSet }

  pub fn label_attribute_type(&self) -> LabelAttributeTypeSet { LabelAttributeTypeSet }

  pub fn label_attribute_type_allowed_value(&self) -> LabelAttributeTypeAllowedValueSet { LabelAttributeTypeAllowedValueSet }

  pub fn label_gid_redirect(&self) -> LabelGidRedirectSet { LabelGidRedirectSet }

  pub fn label_ipi(&self) -> LabelIpiSet { LabelIpiSet }

  pub fn label_isni(&self) -> LabelIsniSet { LabelIsniSet }

  pub fn label_meta(&self) -> LabelMetaSet { LabelMetaSet }

  pub fn label_rating_raw(&self) -> LabelRatingRawSet { LabelRatingRawSet }

  pub fn label_tag(&self) -> LabelTagSet { LabelTagSet }

  pub fn label_tag_raw(&self) -> LabelTagRawSet { LabelTagRawSet }

  pub fn label_type(&self) -> LabelTypeSet { LabelTypeSet }

  pub fn language(&self) -> LanguageSet { LanguageSet }

  pub fn link(&self) -> LinkSet { LinkSet }

  pub fn link_attribute(&self) -> LinkAttributeSet { LinkAttributeSet }

  pub fn link_attribute_credit(&self) -> LinkAttributeCreditSet { LinkAttributeCreditSet }

  pub fn link_attribute_text_value(&self) -> LinkAttributeTextValueSet { LinkAttributeTextValueSet }

  pub fn link_attribute_type(&self) -> LinkAttributeTypeSet { LinkAttributeTypeSet }

  pub fn link_creditable_attribute_type(&self) -> LinkCreditableAttributeTypeSet { LinkCreditableAttributeTypeSet }

  pub fn link_text_attribute_type(&self) -> LinkTextAttributeTypeSet { LinkTextAttributeTypeSet }

  pub fn link_type(&self) -> LinkTypeSet { LinkTypeSet }

  pub fn link_type_attribute_type(&self) -> LinkTypeAttributeTypeSet { LinkTypeAttributeTypeSet }

  pub fn medium(&self) -> MediumSet { MediumSet }

  pub fn medium_attribute(&self) -> MediumAttributeSet { MediumAttributeSet }

  pub fn medium_attribute_type(&self) -> MediumAttributeTypeSet { MediumAttributeTypeSet }

  pub fn medium_attribute_type_allowed_format(&self) -> MediumAttributeTypeAllowedFormatSet { MediumAttributeTypeAllowedFormatSet }

  pub fn medium_attribute_type_allowed_value(&self) -> MediumAttributeTypeAllowedValueSet { MediumAttributeTypeAllowedValueSet }

  pub fn medium_attribute_type_allowed_value_allowed_format(&self) -> MediumAttributeTypeAllowedValueAllowedFormatSet { MediumAttributeTypeAllowedValueAllowedFormatSet }

  pub fn medium_cdtoc(&self) -> MediumCdtocSet { MediumCdtocSet }

  pub fn medium_format(&self) -> MediumFormatSet { MediumFormatSet }

  pub fn medium_index(&self) -> MediumIndexSet { MediumIndexSet }

  pub fn medium_track_durations(&self) -> MediumTrackDurationsSet { MediumTrackDurationsSet }

  pub fn mood(&self) -> MoodSet { MoodSet }

  pub fn mood_alias(&self) -> MoodAliasSet { MoodAliasSet }

  pub fn mood_alias_type(&self) -> MoodAliasTypeSet { MoodAliasTypeSet }

  pub fn mood_annotation(&self) -> MoodAnnotationSet { MoodAnnotationSet }

  pub fn old_editor_name(&self) -> OldEditorNameSet { OldEditorNameSet }

  pub fn orderable_link_type(&self) -> OrderableLinkTypeSet { OrderableLinkTypeSet }

  pub fn place(&self) -> PlaceSet { PlaceSet }

  pub fn place_alias(&self) -> PlaceAliasSet { PlaceAliasSet }

  pub fn place_alias_type(&self) -> PlaceAliasTypeSet { PlaceAliasTypeSet }

  pub fn place_annotation(&self) -> PlaceAnnotationSet { PlaceAnnotationSet }

  pub fn place_attribute(&self) -> PlaceAttributeSet { PlaceAttributeSet }

  pub fn place_attribute_type(&self) -> PlaceAttributeTypeSet { PlaceAttributeTypeSet }

  pub fn place_attribute_type_allowed_value(&self) -> PlaceAttributeTypeAllowedValueSet { PlaceAttributeTypeAllowedValueSet }

  pub fn place_gid_redirect(&self) -> PlaceGidRedirectSet { PlaceGidRedirectSet }

  pub fn place_meta(&self) -> PlaceMetaSet { PlaceMetaSet }

  pub fn place_rating_raw(&self) -> PlaceRatingRawSet { PlaceRatingRawSet }

  pub fn place_tag(&self) -> PlaceTagSet { PlaceTagSet }

  pub fn place_tag_raw(&self) -> PlaceTagRawSet { PlaceTagRawSet }

  pub fn place_type(&self) -> PlaceTypeSet { PlaceTypeSet }

  pub fn recording(&self) -> RecordingSet { RecordingSet }

  pub fn recording_alias(&self) -> RecordingAliasSet { RecordingAliasSet }

  pub fn recording_alias_type(&self) -> RecordingAliasTypeSet { RecordingAliasTypeSet }

  pub fn recording_annotation(&self) -> RecordingAnnotationSet { RecordingAnnotationSet }

  pub fn recording_attribute(&self) -> RecordingAttributeSet { RecordingAttributeSet }

  pub fn recording_attribute_type(&self) -> RecordingAttributeTypeSet { RecordingAttributeTypeSet }

  pub fn recording_attribute_type_allowed_value(&self) -> RecordingAttributeTypeAllowedValueSet { RecordingAttributeTypeAllowedValueSet }

  pub fn recording_first_release_date(&self) -> RecordingFirstReleaseDateSet { RecordingFirstReleaseDateSet }

  pub fn recording_gid_redirect(&self) -> RecordingGidRedirectSet { RecordingGidRedirectSet }

  pub fn recording_meta(&self) -> RecordingMetaSet { RecordingMetaSet }

  pub fn recording_rating_raw(&self) -> RecordingRatingRawSet { RecordingRatingRawSet }

  pub fn recording_series(&self) -> RecordingSeriesSet { RecordingSeriesSet }

  pub fn recording_tag(&self) -> RecordingTagSet { RecordingTagSet }

  pub fn recording_tag_raw(&self) -> RecordingTagRawSet { RecordingTagRawSet }

  pub fn release(&self) -> ReleaseSet { ReleaseSet }

  pub fn release_alias(&self) -> ReleaseAliasSet { ReleaseAliasSet }

  pub fn release_alias_type(&self) -> ReleaseAliasTypeSet { ReleaseAliasTypeSet }

  pub fn release_annotation(&self) -> ReleaseAnnotationSet { ReleaseAnnotationSet }

  pub fn release_attribute(&self) -> ReleaseAttributeSet { ReleaseAttributeSet }

  pub fn release_attribute_type(&self) -> ReleaseAttributeTypeSet { ReleaseAttributeTypeSet }

  pub fn release_attribute_type_allowed_value(&self) -> ReleaseAttributeTypeAllowedValueSet { ReleaseAttributeTypeAllowedValueSet }

  pub fn release_country(&self) -> ReleaseCountrySet { ReleaseCountrySet }

  pub fn release_event(&self) -> ReleaseEventSet { ReleaseEventSet }

  pub fn release_first_release_date(&self) -> ReleaseFirstReleaseDateSet { ReleaseFirstReleaseDateSet }

  pub fn release_gid_redirect(&self) -> ReleaseGidRedirectSet { ReleaseGidRedirectSet }

  pub fn release_group(&self) -> ReleaseGroupSet { ReleaseGroupSet }

  pub fn release_group_alias(&self) -> ReleaseGroupAliasSet { ReleaseGroupAliasSet }

  pub fn release_group_alias_type(&self) -> ReleaseGroupAliasTypeSet { ReleaseGroupAliasTypeSet }

  pub fn release_group_annotation(&self) -> ReleaseGroupAnnotationSet { ReleaseGroupAnnotationSet }

  pub fn release_group_attribute(&self) -> ReleaseGroupAttributeSet { ReleaseGroupAttributeSet }

  pub fn release_group_attribute_type(&self) -> ReleaseGroupAttributeTypeSet { ReleaseGroupAttributeTypeSet }

  pub fn release_group_attribute_type_allowed_value(&self) -> ReleaseGroupAttributeTypeAllowedValueSet { ReleaseGroupAttributeTypeAllowedValueSet }

  pub fn release_group_gid_redirect(&self) -> ReleaseGroupGidRedirectSet { ReleaseGroupGidRedirectSet }

  pub fn release_group_meta(&self) -> ReleaseGroupMetaSet { ReleaseGroupMetaSet }

  pub fn release_group_primary_type(&self) -> ReleaseGroupPrimaryTypeSet { ReleaseGroupPrimaryTypeSet }

  pub fn release_group_rating_raw(&self) -> ReleaseGroupRatingRawSet { ReleaseGroupRatingRawSet }

  pub fn release_group_secondary_type(&self) -> ReleaseGroupSecondaryTypeSet { ReleaseGroupSecondaryTypeSet }

  pub fn release_group_secondary_type_join(&self) -> ReleaseGroupSecondaryTypeJoinSet { ReleaseGroupSecondaryTypeJoinSet }

  pub fn release_group_series(&self) -> ReleaseGroupSeriesSet { ReleaseGroupSeriesSet }

  pub fn release_group_tag(&self) -> ReleaseGroupTagSet { ReleaseGroupTagSet }

  pub fn release_group_tag_raw(&self) -> ReleaseGroupTagRawSet { ReleaseGroupTagRawSet }

  pub fn release_label(&self) -> ReleaseLabelSet { ReleaseLabelSet }

  pub fn release_meta(&self) -> ReleaseMetaSet { ReleaseMetaSet }

  pub fn release_packaging(&self) -> ReleasePackagingSet { ReleasePackagingSet }

  pub fn release_raw(&self) -> ReleaseRawSet { ReleaseRawSet }

  pub fn release_series(&self) -> ReleaseSeriesSet { ReleaseSeriesSet }

  pub fn release_status(&self) -> ReleaseStatusSet { ReleaseStatusSet }

  pub fn release_tag(&self) -> ReleaseTagSet { ReleaseTagSet }

  pub fn release_tag_raw(&self) -> ReleaseTagRawSet { ReleaseTagRawSet }

  pub fn release_unknown_country(&self) -> ReleaseUnknownCountrySet { ReleaseUnknownCountrySet }

  pub fn replication_control(&self) -> ReplicationControlSet { ReplicationControlSet }

  pub fn script(&self) -> ScriptSet { ScriptSet }

  pub fn series(&self) -> SeriesSet { SeriesSet }

  pub fn series_alias(&self) -> SeriesAliasSet { SeriesAliasSet }

  pub fn series_alias_type(&self) -> SeriesAliasTypeSet { SeriesAliasTypeSet }

  pub fn series_annotation(&self) -> SeriesAnnotationSet { SeriesAnnotationSet }

  pub fn series_attribute(&self) -> SeriesAttributeSet { SeriesAttributeSet }

  pub fn series_attribute_type(&self) -> SeriesAttributeTypeSet { SeriesAttributeTypeSet }

  pub fn series_attribute_type_allowed_value(&self) -> SeriesAttributeTypeAllowedValueSet { SeriesAttributeTypeAllowedValueSet }

  pub fn series_gid_redirect(&self) -> SeriesGidRedirectSet { SeriesGidRedirectSet }

  pub fn series_ordering_type(&self) -> SeriesOrderingTypeSet { SeriesOrderingTypeSet }

  pub fn series_tag(&self) -> SeriesTagSet { SeriesTagSet }

  pub fn series_tag_raw(&self) -> SeriesTagRawSet { SeriesTagRawSet }

  pub fn series_type(&self) -> SeriesTypeSet { SeriesTypeSet }

  pub fn tag(&self) -> TagSet { TagSet }

  pub fn tag_relation(&self) -> TagRelationSet { TagRelationSet }

  pub fn track(&self) -> TrackSet { TrackSet }

  pub fn track_gid_redirect(&self) -> TrackGidRedirectSet { TrackGidRedirectSet }

  pub fn track_raw(&self) -> TrackRawSet { TrackRawSet }

  pub fn unreferenced_row_log(&self) -> UnreferencedRowLogSet { UnreferencedRowLogSet }

  pub fn url(&self) -> UrlSet { UrlSet }

  pub fn url_gid_redirect(&self) -> UrlGidRedirectSet { UrlGidRedirectSet }

  pub fn vote(&self) -> VoteSet { VoteSet }

  pub fn work(&self) -> WorkSet { WorkSet }

  pub fn work_alias(&self) -> WorkAliasSet { WorkAliasSet }

  pub fn work_alias_type(&self) -> WorkAliasTypeSet { WorkAliasTypeSet }

  pub fn work_annotation(&self) -> WorkAnnotationSet { WorkAnnotationSet }

  pub fn work_attribute(&self) -> WorkAttributeSet { WorkAttributeSet }

  pub fn work_attribute_type(&self) -> WorkAttributeTypeSet { WorkAttributeTypeSet }

  pub fn work_attribute_type_allowed_value(&self) -> WorkAttributeTypeAllowedValueSet { WorkAttributeTypeAllowedValueSet }

  pub fn work_gid_redirect(&self) -> WorkGidRedirectSet { WorkGidRedirectSet }

  pub fn work_language(&self) -> WorkLanguageSet { WorkLanguageSet }

  pub fn work_meta(&self) -> WorkMetaSet { WorkMetaSet }

  pub fn work_rating_raw(&self) -> WorkRatingRawSet { WorkRatingRawSet }

  pub fn work_series(&self) -> WorkSeriesSet { WorkSeriesSet }

  pub fn work_tag(&self) -> WorkTagSet { WorkTagSet }

  pub fn work_tag_raw(&self) -> WorkTagRawSet { WorkTagRawSet }

  pub fn work_type(&self) -> WorkTypeSet { WorkTypeSet }

}