window.SIDEBAR_ITEMS = {"mod":[["pallet","The module that hosts all the FRAME types needed to add this pallet to a runtime."]],"struct":[["BeefyEquivocationOffence","A BEEFY equivocation offence report."],["BeefyTimeSlot","A round number and set id which point on the time of an offence."],["EquivocationHandler","Generic equivocation handler. This type implements `HandleEquivocation` using existing subsystems that are part of frame (type bounds described below) and will dispatch to them directly, it’s only purpose is to wire all subsystems together."]],"trait":[["BeefyOffence","An interface for types that will be used as BEEFY offences and must also implement the `Offence` trait. This trait provides a constructor that is provided all available data during processing of BEEFY equivocations."],["HandleEquivocation","A trait with utility methods for handling equivocation reports in BEEFY. The offence type is generic, and the trait provides, reporting an offence triggered by a valid equivocation report, and also for creating and submitting equivocation report extrinsics (useful only in offchain context)."],["WeightInfo",""]]};