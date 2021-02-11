
    ;(function() { window.Aura = window.Aura || {}; window.Aura.beforeFrameworkInit = Aura.beforeFrameworkInit || []; window.Aura.beforeFrameworkInit.push(function() { /*
 * This code is for Internal Salesforce use only, and subject to change without notice.
 * Customers shouldn't reference this file in any web pages.
 */
var ContentAssetGlobalValueProvider=function(a,b){this.orgId=a;this.contentDomainUrl=b};ContentAssetGlobalValueProvider.prototype.merge=function(){};ContentAssetGlobalValueProvider.prototype.isStorable=function(){return!1};ContentAssetGlobalValueProvider.prototype.get=function(a){var b="";if(!a||0===a.length)return null;b=a+"?oid\x3d"+this.orgId+"\x26";a="/file-asset/"+b;var b=$A.get("$SfdcSite.pathPrefix"),c=$A.get("$Absolute.url");return b?[c||b||"",a].join(""):[this.contentDomainUrl||"",a].join("")};

//# sourceMappingURL=/javascript/1543290307000/ui-sfdc-javascript-impl/source/ContentAssetGVP.js.map

$A.addValueProvider('$ContentAsset', new ContentAssetGlobalValueProvider('00DU0000000JPkE',''))
 ; }); window.Aura.beforeFrameworkInit.push(function() { /*
 * This code is for Internal Salesforce use only, and subject to change without notice.
 * Customers shouldn't reference this file in any web pages.
 */
var RecordGlobalValueProvider=function(a,b,c,d,e){this._cmp=null;this.configs={refresh:1E3*a,expiration:1E3*b,maxSize:c,version:d,previousVersion:e,minSaveToStorageInterval:3E3};Object.freeze(this.configs)};RecordGlobalValueProvider.prototype.getValues=function(){return{}};RecordGlobalValueProvider.prototype.merge=function(a){$A.util.isEmpty(a)||(this._createCmp(),this._cmp.helper.recordLib.records._receiveFrom$RecordGvp(a))};
RecordGlobalValueProvider.prototype.get=function(a,b){if("configs"===a)return this.configs;this._requestFromServer(a)};RecordGlobalValueProvider.prototype._requestFromServer=function(a){if(this._createCmp()){var b=this._cmp.get("c.getRecord");b.setParams({recordDescriptor:a});b.setCallback(this,$A.getCallback(function(b){"INCOMPLETE"===b.getState()&&this._cmp.helper.handleIncomplete.call(this._cmp.helper,a)}));$A.enqueueAction(b);$A.run(function(){},"RecordGlobalValueProvider._requestFromServer")}};
RecordGlobalValueProvider.prototype._createCmp=function(){this._cmp||(this._cmp=$A.createComponentFromConfig({descriptor:"markup://force:recordGlobalValueProvider",attributes:null}));return!$A.util.isUndefinedOrNull(this._cmp)};

//# sourceMappingURL=/javascript/1515605724000/ui-sfdc-javascript-impl/source/RecordGVP.js.map

$A.addValueProvider('$Record', new RecordGlobalValueProvider(30, 1800, 5120, '50.0', '49.0')) ; });  }());
(function () {
window.pageStartTime = (new Date()).getTime();
window.Aura || (window.Aura = {});
window.Aura.bootstrap || (window.Aura.bootstrap = {});
window.Aura.appBootstrap={"data":{"app":{"componentDef":{"descriptor":"markup://c:FSRegistrationApp"},"creationPath":"/*[0]"}}};
;(function() {
    window.Aura.bootstrap.execBootstrapJs = window.performance && window.performance.now ? window.performance.now() : Date.now();
    window.Aura.appBootstrapStatus = "loaded";
    window.Aura.afterBootstrapReady = window.Aura.afterBootstrapReady || [];
    if (window.Aura.afterBootstrapReady.length) {
        var queue = window.Aura.afterBootstrapReady;
        window.Aura.afterBootstrapReady = [];
        for (var i = 0; i < queue.length; i++) {
           queue[i]();
        }
    }
}());

var time = window.performance && window.performance.now ? window.performance.now.bind(performance) : function(){return Date.now();};
window.Aura.bootstrap.execInlineJs = time();

window.Aura.inlineJsLoaded = true;

var auraConfig = {"deftype":"APPLICATION","ns":{"internal":["adminsuccess","adminui","aloha_sales_forecasting","aloha_sales_opptysplit","aloha_sales_tm2","analytics","analyticsHome","analyzer_framework","appexUi","assistantFramework","assistantFrameworkModules","assistant_builder","aura","auraStorage","auradev","auradocs","auraplat","b2b_buyer_builder","b2b_buyer_cart","b2b_buyer_data","b2b_buyer_dataNamespace","b2b_buyer_error_states","b2b_buyer_navigation","b2b_buyer_orders","b2b_buyer_pricing","b2b_buyer_product_category","b2b_buyer_product_details","b2b_buyer_product_images","b2b_buyer_quick_order","b2b_buyer_styling","b2b_buyer_wishlists","b2b_search_builder","b2b_search_builderNamespace","b2b_search_facets","b2b_search_input","b2b_search_management","b2b_search_managementNamespace","b2b_search_product_card","b2b_search_results_tiles","b2b_search_settings","b2b_search_suggestions","b2b_storefront","b2c_lite_commerce","b2c_lite_template","backToWork","briefcase","builder_communities_nba","builder_framework","builder_industries_dataprocessingengine","builder_industries_fsc","builder_industries_healthcare","builder_industries_insurance","builder_industries_publicsector","builder_industries_survey","builder_industries_utilizationmanagement","builder_industries_visit","builder_platform_blockchain","builder_platform_interaction","builder_platform_process","builder_platform_usage","builder_record_flexipage","builder_service_chatbots","calendar","calendar_view","cg_retail","chatbots","ci_player","clients","clients_chatapp","cmsAuthor","commerce","commerce_catalog","commerce_checkout","commerce_console","commerce_store_integrations","communitySetup","community_article","community_builder","community_byo","community_case","community_cms","community_content","community_dam","community_deflection","community_designtime","community_hierarchy","community_layout","community_login","community_navigation","community_reputation","community_runtime","community_setup","community_styling","community_topic","community_user","community_utils","componentReference","console","contentbuilder","contentpage","conversation","cooper","cordaDashboards","cxm","dashboards","dataImporter","ddcProspector","designtime_journeys","desktopDashboards","eamobile","einstein_assistant","einstein_discovery","einsteinbuilder","einsteinconduit","emailStream","emailui","embeddedService","embedded_service","entityinterface","environmenthub","erb","essentials","essentials_trialassistant","externalServicesSetup","feeds_answer_badging","feeds_autocomplete","feeds_best_answer","feeds_caching","feeds_cases","feeds_compact","feeds_discussion_threading","feeds_emoji","feeds_liking","feeds_metrics","feeds_paging","feeds_placeholding","feeds_post_body_content","feeds_replying","feeds_sorter","feeds_timestamping","feeds_translation","feeds_voting","flexipage","flexipageEditor","flow_analytics","flowruntime","folder","force","forceChatter","forceChatterApi","forceCommunity","forceContent","forceDiscovery","forceHelp","forceKnowledge","forceSearch","forceTopic","formula","frameworkEditor","googleConnector","hammerSetup","healthcloud","hello","home","hvcc","hybrid","industries_common","industries_manufacturing","industries_mfg_common","industries_mfg_forecast","industries_mfg_rebates","industries_mfg_targets","instrumentation","interop","iot","isotope","jsonxform","knowledgeone","laf","lbpm","lcwizard","lightning","lightningInbox","lightningcommunity","lightningcomponentdemo","lightningdocs","lightningsnapin","liveAgent","lst","lsttest","ltng","ltngtools","lwr","macros","mfa_assistant","mulesoft_citizen","myday","native","navex","notes","objectManager","offline","omni","onboarding","onboardingTest","one","onesetup","opencti","packagingSetupUI","platformencryption","process_home","processui","processuiappr","processuicommon","processuimgnt","processuirule","processuitest","record_flexipage","records","reports","runtime_all_walkthroughs","runtime_all_walkthroughsTest","runtime_appointmentbooking","runtime_approval_process","runtime_cdp","runtime_commerce_entitlements","runtime_commerce_ias","runtime_commerce_oms","runtime_commerce_pricing","runtime_commerce_store","runtime_communities_nba","runtime_compliantsharing","runtime_conversation","runtime_einstein_discovery","runtime_essential_checkout","runtime_essentials_common","runtime_essentials_marketing","runtime_essentials_mobile","runtime_essentials_next","runtime_essentials_next_onboarding","runtime_essentials_partner_integrations","runtime_essentials_subscriptions","runtime_hello_studio","runtime_industries_actionplan","runtime_industries_common","runtime_industries_fsc","runtime_industries_healthcare","runtime_industries_insurance","runtime_industries_lending","runtime_industries_loyalty","runtime_industries_publicsector","runtime_industries_referralscoring","runtime_industries_retailexecution","runtime_industries_smartselling","runtime_industries_smarttags","runtime_industries_utilizationmanagement","runtime_industries_visit","runtime_ladybug","runtime_marketing_btobma","runtime_mc2","runtime_mobilesapp","runtime_online_sales","runtime_platform_actions","runtime_platform_employee","runtime_platform_optimizer","runtime_platform_sfdx","runtime_platform_testhistory","runtime_platformservices_condBuilder","runtime_platformservices_transactionSecurity","runtime_process_exception","runtime_qtc_assetmanagement","runtime_quip","runtime_retail_runtime","runtime_rtc","runtime_rtc_spark","runtime_sales_activities","runtime_sales_ade","runtime_sales_billingpayments","runtime_sales_cadence","runtime_sales_cadencebuilder","runtime_sales_campaign","runtime_sales_commerce","runtime_sales_dedupe","runtime_sales_emailtemplateui","runtime_sales_forecasting","runtime_sales_hvs","runtime_sales_insights","runtime_sales_lead","runtime_sales_leadiq","runtime_sales_merge","runtime_sales_multiaddedit","runtime_sales_pathassistant","runtime_sales_pipelineboard","runtime_sales_quotes","runtime_sales_salesAIForEveryone","runtime_sales_see","runtime_sales_skype","runtime_sales_social","runtime_sales_xclean","runtime_search_federated","runtime_service_fieldservice","runtime_service_liveagent","runtime_service_livemessage","runtime_service_objectlinking","runtime_service_omnichannel","runtime_service_predictions","runtime_service_scs","runtime_service_trials","runtime_workdotcom_broadcast","runtime_workdotcom_qmgmt","runtime_workdotcom_scheduling","runtime_workdotcom_trust_cards","s1wizard","sales_einstein","salesforceIdentity","scrt","scrt_setup","search_lightning","securityHealth","securityHub","securitycentral","selfService","serviceCommunity","setup","setupAssistant","setup_analytics_pardot","setup_batch_management","setup_cdp","setup_clients_chatapp","setup_data_translation","setup_document_checklist","setup_einstein_assistant","setup_einstein_shared","setup_industries_common","setup_industries_dataprocessingengine","setup_industries_decisiontable","setup_industries_documentreader","setup_industries_insurance","setup_industries_objectdetection","setup_industries_referralscoring","setup_industries_smartselling","setup_industries_smarttags","setup_lightning_visualforce","setup_mc2","setup_mobile_appclone","setup_mobile_security","setup_osl","setup_osl_actions","setup_platformServices_eventManager","setup_platform_a2","setup_platform_a2Namespace","setup_platform_adoptionapps","setup_platform_adoptionappsNamespace","setup_platform_api_wsdl","setup_platform_cdc","setup_platform_dsar","setup_platform_dsarNamespace","setup_platform_externalconnection","setup_platform_integration","setup_platform_ltngbolt","setup_platform_notifications","setup_platform_optimizer","setup_platform_optimizerNamespace","setup_platform_perms","setup_platform_sfdx","setup_platformservices_customplatform","setup_release_update","setup_sales_einstein","setup_sales_einsteinForecasting","setup_sales_forecasting","setup_sales_insights","setup_sales_leadiq","setup_sales_opportunity_score","setup_sales_pardot","setup_sales_pathassistant","setup_sales_spark","setup_service","setup_service_entityarchiving","setup_service_experience","setup_service_fieldservice","setup_service_intents","setup_service_livemessage","setup_service_messenger","setup_service_objectlinking","setup_service_omnichannel","setup_service_predictions","setup_service_scs","setup_service_smb","setup_service_voice","setup_suggested_articles","setup_suggested_response","setupnav","setupwizard","sfa","sfdc_cms","sfdc_fieldservice","siteforce","siteforceBuilder","slds","support","survey","templatesetup","today","trailheadui","ui","uiExamples","ui_journeys_impl","uns","userProvisioningUI","visualEditor","voice","wave","waveapps","webresources","webruntime_navigation","wfm","wfm_agentengagement","wfm_scheduling","wits","work","workAloha","forceGenerated"],"privileged":["DEVOPSIMPKG1","DEVOPSIMPKG10","DEVOPSIMPKG2","DEVOPSIMPKG3","DEVOPSIMPKG4","DEVOPSIMPKG5","DEVOPSIMPKG6","DEVOPSIMPKG7","DEVOPSIMPKG8","DEVOPSIMPKG9","einsteinservice","et4ae5","FinServ","FinServ_WM","FinServ_RB","FinServ_CB","FinServ_SB","FinServ_WM_SB","FinServ_RB_SB","FinServ_INS_SB","FinServ_CB_SB","FinServ_RB_Pre","FinServ_INS_Pre","FinServ_CB_Pre","FinServWaveExt","FSC","FSC1","FSC10","FSC10gs0","FSC11","FSC12","FSC13","FSC14","FSC15","FSC2","FSC3","FSC4","FSC5","FSC6","FSC6gs0","FSC7","FSC7gs0","FSC8","FSC8gs0","FSC9","FSC9gs0","FSC_RB","fscprerelease","fsc1_r1","fsc2_r1","fsc3_r1","fscwealth","fscwealthE","fscwealthpatch","fscwealthfuture","fscfma","fscwmmain","HealthCloud_SB","HealthCloudGA","hc1_r1","hc2_r1","hc3_r1","HC10gs0","HC11","HC12","HC13","HC14","HC15","HC16","HC17","HC18","HC19","HC20","HC21","HC22","HC23","HC24","HC25","HC4","HC4a","HC5","HC6","HC6gs0","HC7","HC7gs0","HC8","HC8gs0","HC9","HC9gs0","hcfma","hcperf","iqinbox","mcdm_3","mcdm_8","mcdm_15","mcdm_21","mcdm_22","mcdm_23","mcdm_24","mcdm_25","mcdm_26","mcdm_27","mcdm_28","mcdm_29","mcdm_30","mcsocsel","mcsocsel_1","mcsocsel_2","mcsocsel_3","mcsocsel_4","mcsocsel_5","mcsocsel_6","mcsocsel_7","mcsocsel_8","mcsocsel_9","mcsocsel_10","relateiq","wealthone","wealthoneblitz","wealthonep","fscmainstguat","hcmainstguat","fscmainstgpu","hcmainstgpu","fscsbuat","hcsbuat","fscsbpu","hcsbpu","fscr1uat","hcr1uat","fscr1pu","hcr1pu","wkcc"]},"bootstrapInlined":true,"descriptor":"markup://c:FSRegistrationApp","pathPrefix":"/FlexibleScheduler","staticResourceDomain":"","initializers":{"gates":{"scenarioTrackerEnabled.instrumentation.ltng":[0,1],"clientTelemetry.instrumentation.ltng":[1,0],"visualforce.dispatchBlockedEvent":[1,0],"rrh.useLWCRelatedLists":[0,0],"browserIdleTime.instrumentation.ltng":[0,0],"scenarioTrackerMarksEnabled.instrumentation.ltng":[0,1]},"accessChecks":{"ActionCadence.orgHasAccessToCadenceInFolders":false,"S1Desktop.orgHasSimpleRecordHome":false,"Webruntime.orgHasWebruntimePOCEnabled":false,"Interaction.orgHasFlowRuntimeV2":true,"Visualforce.isVFRecordDataInvalidationEnabled":true,"SceFreemium.orgHasFreeScoresEnabled":false,"Interaction.orgHasOfflineFlows":false,"ConsoleNavigation.orgLazyLoadsDetailsTabContent":false,"Support.userHasLightningOpenCtiSettings":false,"Social.hasSocialObjectsPilot":false,"Interaction.orgHasFlowSectionsAndColumns":false,"CMS.orgHasCMSUnlimitedUse":false,"Records.isRRHVersionIndicatorEnabled":false,"Voice.userHasVoiceOutbound":false,"ServiceCloudVoice.userCanToggleCallRecordings":false,"Chatapp.orgCanUseTeamsIntegration":false,"FieldService.orgHasShiftManagement":false,"OrgPermissions.PaymentPlatform":false,"BluetailWits.isSocialInsightsLogoAdmin":false,"Interaction.orgHasFlowRuntimeForceLwcBody":false,"S1Desktop.userHasActivitiesRelatedListsOnRecordHome":false}},"host":"/FlexibleScheduler","auraCmpDefBaseURI":"/auraCmpDef?aura.app=markup://c:FSRegistrationApp&_au=q1pM63bZmm63URqwBU6BUA&_ff=DESKTOP&_l=true&_l10n=en_US&_c=false&_style=-386116354&_density=VIEW_ONE","context":{"mode":"PROD","app":"c:FSRegistrationApp","contextPath":"/FlexibleScheduler","pathPrefix":"/FlexibleScheduler","fwuid":"dDIdorNC3N22LalQ5i3slQ","mlr":1,"uad":1,"loaded":{"APPLICATION@markup://c:FSRegistrationApp":"q1pM63bZmm63URqwBU6BUA"},"globalValueProviders":[{"type":"$SObjectType","values":{"CurrentUser":{"isChatterEnabled":false}}},{"type":"$Locale","values":{"userLocaleLang":"en","userLocaleCountry":"US","language":"en","country":"US","variant":"","langLocale":"en_US","nameOfMonths":[{"fullName":"January","shortName":"Jan"},{"fullName":"February","shortName":"Feb"},{"fullName":"March","shortName":"Mar"},{"fullName":"April","shortName":"Apr"},{"fullName":"May","shortName":"May"},{"fullName":"June","shortName":"Jun"},{"fullName":"July","shortName":"Jul"},{"fullName":"August","shortName":"Aug"},{"fullName":"September","shortName":"Sep"},{"fullName":"October","shortName":"Oct"},{"fullName":"November","shortName":"Nov"},{"fullName":"December","shortName":"Dec"},{"fullName":"","shortName":""}],"nameOfWeekdays":[{"fullName":"Sunday","shortName":"SUN"},{"fullName":"Monday","shortName":"MON"},{"fullName":"Tuesday","shortName":"TUE"},{"fullName":"Wednesday","shortName":"WED"},{"fullName":"Thursday","shortName":"THU"},{"fullName":"Friday","shortName":"FRI"},{"fullName":"Saturday","shortName":"SAT"}],"labelForToday":"Today","firstDayOfWeek":1,"timezone":"America/Chicago","dateFormat":"MMM d, yyyy","shortDateFormat":"M/d/yyyy","longDateFormat":"MMMM d, yyyy","datetimeFormat":"MMM d, yyyy h:mm:ss a","shortDatetimeFormat":"M/d/yyyy h:mm a","timeFormat":"h:mm:ss a","shortTimeFormat":"h:mm a","numberFormat":"#,##0.###","decimal":".","grouping":",","zero":"0","percentFormat":"#,##0%","currencyFormat":"¤#,##0.00;(¤#,##0.00)","currencyCode":"USD","currency":"$","dir":"ltr","lang":"en-US","isEasternNameStyle":false}},{"type":"$Browser","values":{"containerVersion":"","isWEBKIT":true,"isIE11":false,"formFactor":"DESKTOP","isIE10":false,"isContainer":false,"isBlackBerry":false,"isIE7":false,"isIE6":false,"isIE9":false,"isIE8":false,"isDesktop":true,"isTablet":false,"isIPad":false,"isSameSiteNoneIncompatible":false,"isWindowsTablet":false,"isPhone":false,"S1Features":{"isOfflineEnabled":true,"isNativePrimingEnabled":true,"areOfflineDraftsEnabled":false,"isAsyncSaveEnabled":false,"isOfflineQuickActionDraftsEnabled":false,"isMobileNavPageRefEnabled":true,"isPrimingPerfTestModeEnabled":false,"hasLightningOnMobile":true,"isUitrkLoggingEnabled":true,"isAuraParallelBootstrapLoadEnabled":true,"useNativeScroller":false,"isTodayPreviewEnabled":false,"isEclairAdvancedFeaturesEnabled":false,"isSFXUrlFormatSupported":false,"isEncryptedStorageEnabled":true,"isAccountSuggestionsEnabled":false,"isAccountLogoEnabled":false,"isSFXInlineEditListViewEnabled":true,"isLVMInAppBuilderEnabled":true,"RelatedListAdvancedGrid":true,"RelatedListSmoothScrollingEnabled":false,"isRelatedListSmoothScrollingDisabled":false,"RelatedListLWCEnabled":false,"isAllActionsForMruListsOn":false,"isListViewGroupShareEnabled":true,"isLightningConsoleSplitViewEnabled":false,"isSocialInsightsLogoAdmin":false,"shouldShowEinsteinInsightsHome":false,"homeAssistantCollapseCards":false,"isPersonAccountsEnabled":false,"isPersonAccountsApiLightningEnabled":false,"isFilesSharingPrivacyEnabled":true,"isFlexipageActionSchedulingEnabled":true,"orgHasEinsteinInsightsEnabled":false,"orgHasLightningLiveAgent":false,"isExecuteHotSpotForLDS":false,"isInvalidRecordPlatformEventEnabled":false,"isLDSRecordsDebug":false,"forceReloadRecordForInlineEdit":false,"calendarAnythingAllowed":true,"isMultiUserOrgEnabled":true,"isEinsteinAssistantEnabled":false,"isEinsteinAssistantPrefEnabled":false,"isElegibleForEinsteinGlobalModelsEnabled":false,"isEinsteinBuilderEnabled":false,"isEinsteinBuilderLicenseEnabled":false,"isEinsteinBuilderPrefEnabled":false,"isEinsteinBuilderStartedPrefEnabled":false,"isEinsteinBuilderStreamingPilotEnabled":false,"isEinsteinBuilderFreemiumPlusEnabled":true,"isEinsteinBuilderMLMigrated":true,"isEinsteinEverywhereEnabled":false,"isErbPrefEnabled":false,"isErbStartedPrefEnabled":false,"isAiEverywhereEnabled":false,"isEinsteinMsaAcceptedEnabled":false,"isEnhancedTemplatesEnabled":false,"orgHasMatchedLeadsEnabled":false,"isForceRefreshViewEnabled":false,"useRaptorAlohaPage":true,"isEngagementScoreVisible":false,"isLeadScoreVisible":false,"useLexiCallbackReturnUrl":true,"isTaskListViewsDisabled":false,"useLightningReportBuilder":false,"isReportEnhancedRunPageEnabled":true,"isReportEnhancedRunPageCopyToClipboardActionEnabled":false,"recordChangeCapture":false,"isMergeCollisionEnabled":false,"showTaskInfoPopover":true,"isRaptorActionsContainerEnabled":true,"isRaptorActionsContainerEnabledForAccount":true,"isRaptorActionsContainerEnabledForContact":true,"isRaptorActionsContainerEnabledForLead":true,"isRaptorActionsContainerEnabledForOpportunity":true,"isRaptorActionsContainerEnabledForSupport":true,"isHeadlessFooterEnabled":false,"isRRHVersionIndicatorEnabled":false,"isStackedAddressableModalsEnabled":false,"isFormsEnabled":false,"isChatterFeedOnlyInConsole":false,"isCompactFeedInSfx":false,"hasAutoTransitionPerm":false,"hasOptedOutAutoTransition":false,"hasFinishedAutoTransition":false,"isScheduledSwitcherEnabled":true,"isScheduledSwitcherFrequencyDaily":false,"refreshOnInvalidSession":false,"isSplitViewOnStandardEnabled":true,"isLWCDeepCloneEnabled":true,"isLWCRLHeaderActionsEnabled":false,"areNewRecordCallbacksEnabled":false,"isAppNavCapabilitiesEnabled":false,"isScopedModalsOff":false,"isUtilityBarRightAligned":false,"isRRHInConsoleOff":false,"isLWCPinnedRegionTemplatesEnabled":true,"isCollapsiblePinnedRegionsEnabled":false,"isBackgroundNavigationsEnabled":true,"ignoreQueryParamWhitelist":false,"useDoubleIframe":false,"isLVMPinnedListOff":false,"isLVMRaptorHeaderOff":false,"isTemplateEnhancedFolderPrefEnabled":true,"orgCanAccessLtngTempFolder":true,"userCanUseContentBuilder":false,"isSCEFreemiumEnabled":false,"isOpptyScoreEnabled":false,"isOpptyScoreUserPermEnabled":false,"hasHvsBulkEmailWq":false,"hasAccessToCadenceWho":false,"orgHasAccessToCadenceInFolders":false,"orgHasAccessToCadenceListViews":false,"hasAccessToEngagementAlerts":false,"isLexEndUserNoSwitchingEnabled":false,"isBootstrapManagementEnabled":false,"isSmartStoreAdapterV2Enabled":false,"isMobileLwcOfflineEnabled":false,"isLwcFlexipageAllObjMblRH":false,"isFeedFavoritesEnabled":false,"requireOpportunityProducts":false,"isFeedNavBarTabsEnabled":false,"isEditInSubtabEnabled":false,"isCommEnhancedReportRunPageEnabled":true,"canUserCreateReportsInCommunity":false,"isNativeEmailClientEnabled":false,"isLightningBlankShieldEnabled":false,"isMobileMultiStackEngineDisabled":false,"isMobileVFMultiStackEngineEnabled":false,"isLightningExtAllowed":true,"isLightningExtLinkGrabberAllowed":true,"isLightningExtDarkModeAllowed":false,"isLightningExtDarkModeEditable":false,"isLightningExtCmpCustomizationAllowed":true,"isLightningExtRelatedListsAllowed":false,"isLightningExtRequiredFieldsAllowed":false,"isLightningExtInlineEditModifierAllowed":false,"isLightningExtTrailheadAllowed":false,"isLightningExtReleased":true,"isLightningExtRelatedListsReleased":false,"isLightningExtRequiredFieldsReleased":false,"isLightningExtInlineEditModifierReleased":false,"isLightningExtTrailheadReleased":false,"isSimpleRecordHomeEnabled":false,"orgHasShiftManagement":false,"minimumInstrumentationEnabled":false,"LWCMarksEnabled":false,"forceRecordMarksEnabled":false,"forceRecordTransactionsDisabled":false,"pageTrackerTransactionsDisabled":false,"isShowViewHierarchyLinkEnabled":true,"isMultiUserLiveRecordsEnabled":false,"isFieldInstancesEnabled":true,"orgCanCustomizeEntityInterface":false,"orgCanViewBackupMappingUI":false,"useNativeUiScroller":false,"isForecastingMobileLWCViewEnabled":true,"isLwcCreateEditCloneEnabled":true,"isEinsteinDocumentReaderPrefEnabled":false,"isActionConfigurationEnabledInNative":true,"isExpressionsEnabled":false,"canUserShareAnalyticsImages":true,"orgCanDisplayInitialLoadingSpinner":false},"isFIREFOX":false,"isWindowsPhone":false,"isOSX":true,"containerVersionMajor":0,"isAndroid":false,"isIPhone":false,"isIOS":false}},{"type":"$SfdcSite","values":{"pathPrefix":"/FlexibleScheduler"}},{"type":"$LightningContainer","values":{"containedContentHost":"heb.secure.force.com","communityPrefix":"/FlexibleScheduler"}},{"type":"$Global","values":{"eswConfigDeveloperName":{"writable":true,"defaultValue":""},"isVoiceOver":{"writable":true,"defaultValue":false},"setupAppContextId":{"writable":true,"defaultValue":""},"density":{"writable":true,"defaultValue":""},"srcdoc":{"writable":false,"defaultValue":false},"appContextId":{"writable":true,"defaultValue":""},"dynamicTypeSize":{"writable":true,"defaultValue":""}}},{"type":"$Label","values":{"SaveDraftErrors":{"DraftsDisabledDueToIssue":"Offline Edit has been disabled due to an issue with your device. Contact Support for help.","Generic":"There was an error saving your pending change. Please try again or contact Salesforce for help.","PendingSync":"Offline records can't be edited while your device is syncing to Salesforce.","DeleteEditDraft":"You can't delete this record while create or edit is pending.","DeleteIrreconcilableDraft":"You can't delete this record while an unresolvable error is present.","NoDraftLookupToDraftId":"You can't save this record with a reference to a record created offline.","DifferentActionThanDraft":"You've already edited this record using a different action. To save this record, make the changes again with the first action.","EditDeleteDraft":"You can't edit this record while deletion is pending."},"Errors":{"NoRecordDataFound":"We couldn't find the record you're trying to access. It may have been deleted by another user, or there may have been a system error. Ask your administrator for help."},"ForceRecord":{"RecordDataCannotUseEntity":"This entity is not currently supported by force:recordData.","invalidRecordLibraryUse":"This application contains a reference to the force:record component, which is not supported by this application."},"Offline":{"NoConnectionCSRFProblemTitle":"$Label.Offline.NoConnectionCSRFProblemTitle does not exist.","NoConnectionTitle":"No Network Connection"},"Duration":{"secondsLater":"in a few seconds","secondsAgo":"a few seconds ago"},"Exception":{"NoAccessException_desc":"You do not have the level of access necessary to perform the operation you requested. Please contact the owner of the record or your administrator if access is necessary."}}}],"enableAccessChecks":true,"apce":1,"apck":"z4kio3kENbSv_KBl4xL80g","dns":"c","ls":1,"lv":"50","mna":{"lightning":"interop"},"arse":1,"services":["markup://lightning:configProvider","markup://force:wireServiceSfdc","markup://force:salesforceScopedModuleResolver","markup://instrumentation:locatorService"]},"attributes":{"ltngOut":"true"},"MaxParallelXHRCount":6,"XHRExclusivity":false,"applyCssVarPolyfill":false};
auraConfig.context.styleContext = {"c":"webkit","x":["isDesktop"],"tokens":["markup://force:sldsTokens","markup://force:base","markup://force:formFactorLarge"],"tuid":"IyL8hvirTasQ7PTkTTayug","cuid":-386116354};

function auraPreInitBlock () {
	
}

function initFramework () {
	window.Aura = window.Aura || {};
	window.Aura.app = auraConfig["context"]["app"];
	window.Aura.beforeFrameworkInit = Aura.beforeFrameworkInit || [];
	window.Aura.beforeFrameworkInit.push(auraPreInitBlock);
	window.Aura.inlineJsReady = time();

	if (!window.Aura.frameworkJsReady) {
		window.Aura.initConfig = auraConfig;
	} else {

		// Set a data attribute on document.body to signal engine to bypass global patching
		if (auraConfig.context && !!auraConfig.context["dpbp"]) {
			document.body.setAttribute("data-global-patching-bypass", "temporary-bypass");
		}

		// LockerService must be initialized before scripts can be executed.
		$A.lockerService.initialize(auraConfig.context);

		// scripts inside custom templates with Locker active are stored
		// until now since we need LockerService before running

		var scripts = window.Aura.inlineJsLocker;
		if (scripts) {
			for (var i = 0; i < scripts.length; i++) {
				$A.lockerService.runScript(scripts[i]["callback"], scripts[i]["namespace"]);
			}
			delete window.Aura.inlineJsLocker;
		}

		if (false) {
			$A.initAsync(auraConfig);
		} else if (false) {
			$A.initConfig(auraConfig);
		}
	}
}

// Error msg
var x = document.getElementById("dismissError");
if (x) {
	x.addEventListener("click", function () {
		var auraErrorMask = document.getElementById("auraErrorMask");
		if (window["$A"]) {
			$A.util.removeClass(auraErrorMask, "auraForcedErrorBox");
			$A.util.removeClass($A.util.getElement("auraErrorReload"), "show");
		} else {
			document.body.removeChild(auraErrorMask);
		}
	});
}

setTimeout(initFramework, 0); // ensure async



var appCssLoadingCount = 0;
function onLoadStyleSheets(event) {
	if(event){
		var element = event.target;
		element.removeEventListener("load",onLoadStyleSheets);
		element.removeEventListener("error",onLoadStyleSheetsError);
	}
	window.Aura.bootstrap.appCssLoading = (--appCssLoadingCount) > 0;
	if (!window.Aura.bootstrap.appCssLoading) {
		if (typeof window.Aura.bootstrap.appCssLoadedCallback === "function") {
			window.Aura.bootstrap.appCssLoadedCallback();
			window.Aura.bootstrap.appCssLoadedCallback = undefined;
		}
	}
}
function onLoadStyleSheetsError(event) {
	window.Aura.bootstrap.hasCssLoadError = true;
	onLoadStyleSheets(event);
}

var auraCss = document.getElementsByClassName("auraCss");
var current;
window.Aura.bootstrap.appCssLoading = auraCss.length > 0;
for (var c=0,length=auraCss.length;c<length;c++) {
	current = auraCss[c];
	appCssLoadingCount++;
	if(auraConfig.applyCssVarPolyfill) {
		loadViaAjax(current, auraConfig.cssVariables);
	} else {
		current.addEventListener("load",onLoadStyleSheets);
		current.addEventListener("error",onLoadStyleSheetsError);
		current.href = current.getAttribute("data-href");
	}
}

function rewriteCssVars(css, varLookup) {
	var VAR_BEGINNING = "var(--",
		VAR_PATTERN = /var\(--[^-]+-(.*?)\)/g,
		startIndex = 0,
		output = [],
		result;

	function processInContext(css, start, delimiter) {
		while( start++ && start < css.length) {
			if(css[start] === delimiter) {

				return start;
			}
		}
	}

	function processParenthesesInContext(css, start) {
		var opens = 0; var closes = 0;
		while(start < css.length) {
			// This allows us to support nested variables.
			// Count "open" and "close" parentheses. These must match before we return.
			// Note: this assumes parentheses formatting is valid.
			if (css[start] === "(") { opens++; }
			else if (css[start] === ")") { closes++; }
			if(opens === closes) {
				return start;
			}
			start++;
		}
	}

	function findVarEnd(css) {
		for(var idx = VAR_BEGINNING.length; idx < css.length; idx++) {
			switch(css[idx]) {
				case "'":
					idx = processInContext(css, idx, "'");
					break;
				case '"':
					idx = processInContext(css, idx, '"');
					break;
				case "(":
					// we process parentheses differently because open/close must match
					idx = processParenthesesInContext(css, idx);
					break;
				case ")":
					//end of var
					return idx + 1;
				default:
				//continue
			}
		}
	}

	function extractValue(declaration, lookup) {
		var parts = declaration.split(",");
		// try to get value from the lookup
		if(lookup) {
			var match = /var\(--(.*)(?:$|\))/.exec(parts[0]);
			if(match && match.length > 1) {
				var value = lookup[match[1]];
				if(value) {
					return value;
				}
			}
		}
		// get hard coded fallback value
		if(parts.length > 1) {
			var val = declaration.substring(declaration.indexOf(",") + 1);
			var fallback = val.substring(0, val.length-1);

			// if the fallback is another variable, resolve the value recursively
			if (fallback.indexOf("var(") != -1) {
				return extractValue(fallback, lookup);
			}
			return fallback;
		}
	}

	function replaceBlobUrlSubresources(css) {
		var anchor = document.createElement("a");
		return css.replace(/\b(url\s*\(\s*['"]?)([^)"']+)(['"]?\))/g, function (match, left, url, right) {
			anchor.href = url;
			return left + anchor.href + right;
		});
	}

	while ( (result = VAR_PATTERN.exec(css)) ) {

		output.push(css.substring(startIndex, result.index));
		startIndex = result.index;

		var endIndex = startIndex + findVarEnd(css.substring(startIndex));
		var declaration = css.substring(startIndex, endIndex);
		var value = extractValue(declaration, varLookup);

		if(value) {
			output.push(value);
		} else {
			output.push(declaration);
		}
		startIndex = endIndex;
	}

	if(startIndex === 0) {
		return replaceBlobUrlSubresources(css);
	}

	output.push(css.substring(startIndex));

	return replaceBlobUrlSubresources(output.join(""));
}

function injectStyles(linkEl,css){
	var cssEl;
	
	if(auraConfig.applyCssVarPolyfillViaBlob) {
		var blob=new Blob([css],{type:"text/css"});
		cssEl=document.createElement("link");
		cssEl.addEventListener("load",onLoadStyleSheets);
		cssEl.addEventListener("error",onLoadStyleSheetsError);
		cssEl.setAttribute("rel","stylesheet");
		cssEl.setAttribute("href",URL.createObjectURL(blob));
		linkEl.parentElement.insertBefore(cssEl,linkEl);
	}else{
		cssEl=document.createElement("style");
		cssEl.textContent=css;
		linkEl.parentElement.insertBefore(cssEl,linkEl);
		onLoadStyleSheets();
	}
}

function rewriteAndInjectCss(linkEl, source, varLookup) {
	var css = rewriteCssVars(source, varLookup);
	injectStyles(linkEl, css);
}

function loadViaAjax(linkEl, cssVariables) {
	var url = linkEl.getAttribute("data-href");
	var xhr = new XMLHttpRequest();
	xhr.addEventListener("error", onLoadStyleSheetsError);
	xhr.addEventListener("load", function (e) {
		if (this.status === 200) {
			rewriteAndInjectCss(linkEl, xhr.responseText, cssVariables);
		} else {
			onLoadStyleSheets(e);
		}
	});
	xhr.open("GET", url);
	xhr.send();
}

window.Aura.rewriteCssVars = rewriteCssVars;
}());
    