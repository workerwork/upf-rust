//定义信元枚举
#[derive(Debug)]
pub enum IEType {
    //Grouped IE, extendable
    CreatePDR = 1,
    PDI = 2,
    CreateFAR = 3,
    ForwardingParameters = 4,
    DuplicatingParameters = 5,
    CreateURR = 6,
    CreateQER = 7,
    CreatedPDR = 8,
    UpdatePDR = 9,
    UpdateFAR = 10,
    UpdateForwardingParameters = 11,
    UpdateBAR = 12,
    UpdateURR = 13,
    UpdateQER = 14,
    RemovePDR = 15,
    RemoveFAR = 16,
    RemoveURR = 17,
    RemoveQER = 18,
    //
    Cause = 19,
    SourceInterface = 20,
    FTEID = 21,
    NetworkInstance = 22,
    SDFFilter = 23,
    ApplicationID = 24,
    GateStatus = 25,
    MBR = 26,
    GBR = 27,
    QERCorrelationID = 28,
    Precedence = 29,
    TransportLevelMarking = 30,
    VolumeThreshold = 31,
    TimeThreshold = 32,
    MonitoringTime = 33,
    SubsequentVolumeThreshold = 34,
    SubsequentTimeThreshold = 35,
    InactivityDetectionTime = 36,
    ReportingTriggers = 37,
    RedirectInformation = 38,
    ReportType = 39,
    OffendingIE = 40,
    ForwardingPolicy = 41,
    DestinationInterface = 42,
    UPFunctionFeatures = 43,
    ApplyAction = 44,
    DownLinkDataServiceInformation = 45,
    DownLinkDataNotificationDelay = 46,
    DLBufferingDuration = 47,
    DLBufferingSuggestedPacketCount = 48,
    PFCPSMReqFlags = 49,
    PFCPSRRspFlags = 50,
    SequenceNumber = 52,
    Metric = 53,
    Timer = 55,
    PDRID = 56,
    FSEID = 57,
    NodeID = 60,
    PFDContents = 61,
    MeasurementMethod = 62,
    UsageReportTrigger = 63,
    MeasurementPeriod = 64,
    FullyQualifiedPDNConnectionSetIdentifier = 65,
    VolumeMeasurement = 66,
    DurationMeasurement = 67,
    TimeOfFirstPacket = 69,
    TimeOfLastPacket = 70,
    QuotaHoldingTime = 71,
    DroppedDLTrafficThreshold = 72,
    VolumeQuota = 73,
    TimeQuota = 74,
    StartTime = 75,
    EndTime = 76,
    URRID = 81,
    LinkedURRID = 82,
    OuterHeaderCreation = 84,
    BARID = 88,
    CPFunctionFeatures = 89,
    UsageInformation = 90,
    ApplicationInstanceID = 91,
    FlowInformation = 92,
    UEIPAddress = 93,
    PacketRate = 94,
    OuterHeaderRemoval = 95,
    RecoveryTimeStamp = 96,
    DLFlowLevelMarking = 97,
    HeaderEnrichment = 98,
    MeasurementInformation = 100,
    NodeReportType = 101,
    RemoteGTPUPeer = 103,
    URSEQN = 104,
    ActivatePredefinedRules = 106,
    DeactivatePredefinedRules = 107,
    FARID = 108,
    QERID = 109,
    OCIFlags = 110,
    PFCPAssociationReleaseRequest = 111,
    GracefulReleasePeriod = 112,
    PDNType = 113,
    FailedRuleID = 114,
    TimeQuotaMechanism = 115,
    UserPlaneIPResourceInformation = 116,
    UserPlaneInactivityTimer = 117,
    Multiplier = 119,
    AggregatedURRID = 120,
    SubsequentVolumeQuota = 121,
    SubsequentTimeQuota = 122,
    RQI = 123,
    QFI = 124,
    QueryURRReference = 125,
    AdditionalUsageReportsInformation = 126,
    TrafficEndpointID = 131,
    MACAddress = 133,
    CTAG = 134,
    STAG = 135,
    EtherType = 136,
    Proxying = 137,
    EthernetFilterID = 138,
    EthernetFilterProperties = 139,
    SuggestedBufferingPacketsCount = 140,
    UserID = 141,
    EthernetPDUSessionInformation = 142,
    MACAddressesDetected = 144,
    MACAddressesRemoved = 145,
    EthernetInactivityTimer = 146,
    EventQuota = 148,
    EventThreshold = 149,
    SubsequentEventQuota = 150,
    SubsequentEventThreshold = 151,
    TraceInformation = 152,
    FramedRoute = 153,
    FramedRouting = 154,
    FramedIPv6Route = 155,
    EventTimeStamp = 156,
    AveragingWindow = 157,
    PagingPolicyIndicator = 158,
    APNDNN = 159,
    _3GPPInterfaceType = 160,
    PFCPSRReqFlags = 161,
    PFCPAUReqFlags = 162,
    ActivationTime = 163,
    DeactivationTime = 164,
    MARID = 170,
    SteeringFunctionality = 171,
    SteeringMode = 172,
    Weight = 173,
    Priority = 174,
    UEIPAddressPoolIdentity = 177,
    AlternativeSMFIPAddress = 178,
    PacketReplicationAndDetectionCarryOnInformation = 179,
    SMFSetID = 180,
    QuotaValidityTime = 181,
}
