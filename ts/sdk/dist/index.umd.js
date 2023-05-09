/*!
 * mitosis-cw-contract-sdk v 0.0.1
 * (c) hashableric <hashableric@gmail.com>
 * Released under the MIT OR Apache-2.0 License.
 */

(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
    typeof define === 'function' && define.amd ? define(['exports'], factory) :
    (global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global["counter-sdk"] = {}));
})(this, (function (exports) { 'use strict';

    /******************************************************************************
    Copyright (c) Microsoft Corporation.

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
    REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
    AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
    INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
    LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
    OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
    PERFORMANCE OF THIS SOFTWARE.
    ***************************************************************************** */
    /* global Reflect, Promise */

    var extendStatics = function(d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };

    function __extends(d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    }

    var __assign = function() {
        __assign = Object.assign || function __assign(t) {
            for (var s, i = 1, n = arguments.length; i < n; i++) {
                s = arguments[i];
                for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
            }
            return t;
        };
        return __assign.apply(this, arguments);
    };

    function __awaiter(thisArg, _arguments, P, generator) {
        function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
        return new (P || (P = Promise))(function (resolve, reject) {
            function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
            function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
            function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
            step((generator = generator.apply(thisArg, _arguments || [])).next());
        });
    }

    function __generator(thisArg, body) {
        var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
        return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
        function verb(n) { return function (v) { return step([n, v]); }; }
        function step(op) {
            if (f) throw new TypeError("Generator is already executing.");
            while (g && (g = 0, op[0] && (_ = 0)), _) try {
                if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
                if (y = 0, t) op = [op[0] & 2, t.value];
                switch (op[0]) {
                    case 0: case 1: t = op; break;
                    case 4: _.label++; return { value: op[1], done: false };
                    case 5: _.label++; y = op[1]; op = [0]; continue;
                    case 7: op = _.ops.pop(); _.trys.pop(); continue;
                    default:
                        if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                        if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                        if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                        if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                        if (t[2]) _.ops.pop();
                        _.trys.pop(); continue;
                }
                op = body.call(thisArg, _);
            } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
            if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
        }
    }

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _0 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var DenommanagerQueryClient = /** @class */ (function () {
        function DenommanagerQueryClient(client, contractAddress) {
            var _this = this;
            this.getConfig = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_config: {}
                        })];
                });
            }); };
            this.convert = function (_a) {
                var token = _a.token;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                convert: {
                                    token: token
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.getConfig = this.getConfig.bind(this);
            this.convert = this.convert.bind(this);
        }
        return DenommanagerQueryClient;
    }());
    var DenommanagerClient = /** @class */ (function (_super) {
        __extends(DenommanagerClient, _super);
        function DenommanagerClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.addAlias = function (_a, fee, memo, funds) {
                var denom = _a.denom, token = _a.token;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    add_alias: {
                                        denom: denom,
                                        token: token
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.changeOwner = function (_a, fee, memo, funds) {
                var newOwner = _a.newOwner;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    change_owner: {
                                        new_owner: newOwner
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.grantRole = function (_a, fee, memo, funds) {
                var addr = _a.addr, role = _a.role;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    grant_role: {
                                        addr: addr,
                                        role: role
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.revokeRole = function (_a, fee, memo, funds) {
                var addr = _a.addr, role = _a.role;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    revoke_role: {
                                        addr: addr,
                                        role: role
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.pause = function (_a, fee, memo, funds) {
                var expiresAt = _a.expiresAt;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    pause: {
                                        expires_at: expiresAt
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.release = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    release: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.addAlias = _this.addAlias.bind(_this);
            _this.changeOwner = _this.changeOwner.bind(_this);
            _this.grantRole = _this.grantRole.bind(_this);
            _this.revokeRole = _this.revokeRole.bind(_this);
            _this.pause = _this.pause.bind(_this);
            _this.release = _this.release.bind(_this);
            return _this;
        }
        return DenommanagerClient;
    }(DenommanagerQueryClient));

    var _1 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        DenommanagerQueryClient: DenommanagerQueryClient,
        DenommanagerClient: DenommanagerClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _2 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var GatewayQueryClient = /** @class */ (function () {
        function GatewayQueryClient(client, contractAddress) {
            var _this = this;
            this.getConfig = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_config: {}
                        })];
                });
            }); };
            this.client = client;
            this.contractAddress = contractAddress;
            this.getConfig = this.getConfig.bind(this);
        }
        return GatewayQueryClient;
    }());
    var GatewayClient = /** @class */ (function (_super) {
        __extends(GatewayClient, _super);
        function GatewayClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.changeOwner = function (_a, fee, memo, funds) {
                var newOwner = _a.newOwner, newPublicKey = _a.newPublicKey;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    change_owner: {
                                        new_owner: newOwner,
                                        new_public_key: newPublicKey
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.changeLiquidityManager = function (_a, fee, memo, funds) {
                var newLiquidityManager = _a.newLiquidityManager;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    change_liquidity_manager: {
                                        new_liquidity_manager: newLiquidityManager
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.changeDenomManager = function (_a, fee, memo, funds) {
                var newDenomManager = _a.newDenomManager;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    change_denom_manager: {
                                        new_denom_manager: newDenomManager
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.pause = function (_a, fee, memo, funds) {
                var expiresAt = _a.expiresAt;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    pause: {
                                        expires_at: expiresAt
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.send = function (_a, fee, memo, funds) {
                var opArgs = _a.opArgs, opId = _a.opId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    send: {
                                        op_args: opArgs,
                                        op_id: opId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.execute = function (_a, fee, memo, funds) {
                var msgs = _a.msgs, signature = _a.signature;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    execute: {
                                        msgs: msgs,
                                        signature: signature
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.release = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    release: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.changeOwner = _this.changeOwner.bind(_this);
            _this.changeLiquidityManager = _this.changeLiquidityManager.bind(_this);
            _this.changeDenomManager = _this.changeDenomManager.bind(_this);
            _this.pause = _this.pause.bind(_this);
            _this.send = _this.send.bind(_this);
            _this.execute = _this.execute.bind(_this);
            _this.release = _this.release.bind(_this);
            return _this;
        }
        return GatewayClient;
    }(GatewayQueryClient));

    var _3 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        GatewayQueryClient: GatewayQueryClient,
        GatewayClient: GatewayClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */

    var _4 = /*#__PURE__*/Object.freeze({
        __proto__: null
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    var LiquiditymanagerQueryClient = /** @class */ (function () {
        function LiquiditymanagerQueryClient(client, contractAddress) {
            var _this = this;
            this.getConfig = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            get_config: {}
                        })];
                });
            }); };
            this.pauseInfo = function () { return __awaiter(_this, void 0, void 0, function () {
                return __generator(this, function (_a) {
                    return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                            pause_info: {}
                        })];
                });
            }); };
            this.getBalance = function (_a) {
                var depositor = _a.depositor;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_balance: {
                                    depositor: depositor
                                }
                            })];
                    });
                });
            };
            this.getBond = function (_a) {
                var bonder = _a.bonder;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_bond: {
                                    bonder: bonder
                                }
                            })];
                    });
                });
            };
            this.getUnbond = function (_a) {
                var unbondId = _a.unbondId;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_unbond: {
                                    unbond_id: unbondId
                                }
                            })];
                    });
                });
            };
            this.getUnbondsByOwner = function (_a) {
                var owner = _a.owner;
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        return [2 /*return*/, this.client.queryContractSmart(this.contractAddress, {
                                get_unbonds_by_owner: {
                                    owner: owner
                                }
                            })];
                    });
                });
            };
            this.client = client;
            this.contractAddress = contractAddress;
            this.getConfig = this.getConfig.bind(this);
            this.pauseInfo = this.pauseInfo.bind(this);
            this.getBalance = this.getBalance.bind(this);
            this.getBond = this.getBond.bind(this);
            this.getUnbond = this.getUnbond.bind(this);
            this.getUnbondsByOwner = this.getUnbondsByOwner.bind(this);
        }
        return LiquiditymanagerQueryClient;
    }());
    var LiquiditymanagerClient = /** @class */ (function (_super) {
        __extends(LiquiditymanagerClient, _super);
        function LiquiditymanagerClient(client, sender, contractAddress) {
            var _this = _super.call(this, client, contractAddress) || this;
            _this.deposit = function (_a, fee, memo, funds) {
                var depositor = _a.depositor;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    deposit: {
                                        depositor: depositor
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.withdraw = function (_a, fee, memo, funds) {
                var amount = _a.amount, withdrawer = _a.withdrawer;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    withdraw: {
                                        amount: amount,
                                        withdrawer: withdrawer
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.delegate = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    delegate: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.undelegate = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    undelegate: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.bond = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    bond: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.startUnbond = function (_a, fee, memo, funds) {
                var amount = _a.amount;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    start_unbond: {
                                        amount: amount
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.unbond = function (_a, fee, memo, funds) {
                var unbondId = _a.unbondId;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    unbond: {
                                        unbond_id: unbondId
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.changeOwner = function (_a, fee, memo, funds) {
                var newOwner = _a.newOwner;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    change_owner: {
                                        new_owner: newOwner
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.grantRole = function (_a, fee, memo, funds) {
                var addr = _a.addr, role = _a.role;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    grant_role: {
                                        addr: addr,
                                        role: role
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.revokeRole = function (_a, fee, memo, funds) {
                var addr = _a.addr, role = _a.role;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    revoke_role: {
                                        addr: addr,
                                        role: role
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.pause = function (_a, fee, memo, funds) {
                var expiresAt = _a.expiresAt;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    pause: {
                                        expires_at: expiresAt
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.release = function (fee, memo, funds) {
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    release: {}
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _a.sent()];
                        }
                    });
                });
            };
            _this.changeConfig = function (_a, fee, memo, funds) {
                var unbondingPeriod = _a.unbondingPeriod;
                if (fee === void 0) { fee = "auto"; }
                return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0: return [4 /*yield*/, this.client.execute(this.sender, this.contractAddress, {
                                    change_config: {
                                        unbonding_period: unbondingPeriod
                                    }
                                }, fee, memo, funds)];
                            case 1: return [2 /*return*/, _b.sent()];
                        }
                    });
                });
            };
            _this.client = client;
            _this.sender = sender;
            _this.contractAddress = contractAddress;
            _this.deposit = _this.deposit.bind(_this);
            _this.withdraw = _this.withdraw.bind(_this);
            _this.delegate = _this.delegate.bind(_this);
            _this.undelegate = _this.undelegate.bind(_this);
            _this.bond = _this.bond.bind(_this);
            _this.startUnbond = _this.startUnbond.bind(_this);
            _this.unbond = _this.unbond.bind(_this);
            _this.changeOwner = _this.changeOwner.bind(_this);
            _this.grantRole = _this.grantRole.bind(_this);
            _this.revokeRole = _this.revokeRole.bind(_this);
            _this.pause = _this.pause.bind(_this);
            _this.release = _this.release.bind(_this);
            _this.changeConfig = _this.changeConfig.bind(_this);
            return _this;
        }
        return LiquiditymanagerClient;
    }(LiquiditymanagerQueryClient));

    var _5 = /*#__PURE__*/Object.freeze({
        __proto__: null,
        LiquiditymanagerQueryClient: LiquiditymanagerQueryClient,
        LiquiditymanagerClient: LiquiditymanagerClient
    });

    /**
    * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
    * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
    * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
    */
    exports.contracts = void 0;
    (function (contracts) {
        contracts.Denommanager = __assign(__assign({}, _0), _1);
        contracts.Gateway = __assign(__assign({}, _2), _3);
        contracts.Liquiditymanager = __assign(__assign({}, _4), _5);
    })(exports.contracts || (exports.contracts = {}));

    Object.defineProperty(exports, '__esModule', { value: true });

}));
//# sourceMappingURL=index.umd.js.map
