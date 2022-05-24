import {  TokenPayment } from "@elrondnetwork/erdjs";
import { createAirdropService, FiveMinutesInMilliseconds, createESDTInteractor, INetworkProvider, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { helperAddLiquidityPool, helperAirdropTokens, helperIssueBorrowToken, helperIssueLendToken, helperIssueToken, helperSetAssetLiquidationBonus, helperSetAssetLoanToValue, helperSetBorrowRoles, helperSetLendRoles } from "./lendingPoolHelper";
import { createLendingInteractor } from "./lendingPoolInteractor";
import { createLiquidityInteractor } from "./liquidityPoolInteractor";

describe("lending snippet", async function () {
    this.bail(true);

    let suite = this;
    let session: ITestSession;
    let provider: INetworkProvider;
    let whale: ITestUser;
    let firstUser: ITestUser;
    let secondUser: ITestUser;

    this.beforeAll(async function () {
        session = await TestSession.load("devnet", __dirname);
        provider = session.networkProvider;
        whale = session.users.getUser("whale");
        firstUser = session.users.getUser("firstUser");
        secondUser = session.users.getUser("secondUser");
        await session.syncNetworkConfig();
    });

    it("Airdrop EGLD", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        
        let payment = TokenPayment.egldFromAmount(0.1);
        await session.syncUsers([whale]);
        await createAirdropService(session).sendToEachUser(whale, [firstUser, secondUser], [payment]);
    });

    it("Issue Pool Token", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

        let interactor = await createESDTInteractor(session);
        await session.syncUsers([whale]);
        let token = await interactor.issueFungibleToken(whale, { name: "ABC", ticker: "ABC", decimals: 18, supply: "1000000000000000000000" })
        await session.saveToken({ name: "tokenABC", token: token });
    });

    it("Deploy", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

        await session.syncUsers([whale]);

        let token = await session.loadToken("tokenABC");
        let interactor = await createLendingInteractor(session);

        // Deploy dummy liquidity pool
        let { address: dummyAddress, returnCode: dummyReturnCode } = await interactor.deployDummyLiquidityPool(whale, token.identifier);
        assert.isTrue(dummyReturnCode.isSuccess());

        // Deploy lending pool
        let { address, returnCode } = await interactor.deploy(whale, dummyAddress);
        assert.isTrue(returnCode.isSuccess());
        await session.saveAddress({name: "lendingAddr", address: address});
    });


    it("Create Liquidity Pool", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

        await session.syncUsers([whale]);

        let token = await session.loadToken("tokenABC");
        let lendAddress = await session.loadAddress("lendingAddr");
        let interactor = await createLendingInteractor(session, lendAddress);

        // Setup Liquidity pool
        let returnCode = await interactor.addLiquidityPool(whale, token.identifier, 0, 40000000, 1000000000, 800000000, 100000000, 700000000);
        assert.isTrue(returnCode.isSuccess());

        isSuccess = await helperAddLiquidityPool(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);
    });

    it("Issue Lending Token", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

    it("Issue Lend Tokens", async function () {
        session.expectLongInteraction(this);
        await session.syncUsers([whale]);

        let token = await session.loadToken("tokenABC");
        let lendAddress = await session.loadAddress("lendingAddr");
        let interactor = await createLendingInteractor(session, lendAddress);

        isSuccess = helperIssueLendToken(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);
    });

    it("Issue Borrow Token", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

    it("Issue Borrow Tokens ", async function () {
        session.expectLongInteraction(this);
        await session.syncUsers([whale]);

        let token = await session.loadToken("tokenABC");
        let lendAddress = await session.loadAddress("lendingAddr");
        let interactor = await createLendingInteractor(session, lendAddress);

        isSuccess = helperIssueBorrowToken(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);
    });

    it("Setup Lending Pool", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

        await session.syncUsers([whale]);

        let token = await session.loadToken("tokenABC");
        let lendAddress = await session.loadAddress("lendingAddr");
        let interactor = await createLendingInteractor(session, lendAddress);

        isSuccess = helperSetLendRoles(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);

        isSuccess = helperSetBorrowRoles(session, whale, "tokenABC");
        assert.isTrue(isSuccess);

        isSuccess = helperSetBorrowRoles(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);

        isSuccess = helperSetAssetLoanToValue(session, whale, "tokenABC");
        assert.isTrue(isSuccess);

        isSuccess = helperSetAssetLoanToValue(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);

        isSuccess = helperSetAssetLiquidationBonus(session, whale, "tokenABC");
        assert.isTrue(isSuccess);

        isSuccess = helperSetAssetLiquidationBonus(session, whale, "tokenXYZ");
        assert.isTrue(isSuccess);
    });


    it("airdrop pool_token to users", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        this.retries(5);

        let isSuccess = helperAirdropTokens(session, whale, firstUser, secondUser, "tokenABC");
        assert.isTrue(isSuccess);
    });


<<<<<<< HEAD
    it("deposit token", async function () {
        this.timeout(FiveMinutesInMilliseconds);
=======

    it("deposit token ABC", async function () {
        session.expectLongInteraction(this);
        await session.syncUsers([whale, firstUser]);
>>>>>>> Fix Borrow, Withdraw, Deposit scenarios

        let token = await session.loadToken("tokenABC");
        let address = await session.loadAddress("lendingAddr");
        let interactor = await createLendingInteractor(session, address);
        let paymentABC = TokenPayment.fungibleFromAmount(token.identifier, "20", token.decimals);
        let { returnCode: returnCodeDeposit, depositNonce: depositNonceABC } = await interactor.deposit(firstUser, paymentABC);
        assert.isTrue(returnCodeDeposit.isSuccess());

        session.saveBreadcrumb("depositNonceABC", depositNonceABC)
    });


    it("deposit token XYZ", async function () {
        session.expectLongInteraction(this);
        await session.syncUsers([whale, firstUser]);

<<<<<<< HEAD
        session.saveBreadcrumb({ name: "depositNonceOne", value: depositNonceOne})
        session.saveBreadcrumb({name: "depositNonceTwo", value: depositNonceTwo})
    });
=======
        let token = await session.loadToken("tokenXYZ");
        let address = await session.loadAddress("contractAddress");
        let interactor = await createLendingInteractor(session, address);
        let paymentXYZ = TokenPayment.fungibleFromAmount(token.identifier, "20", token.decimals);
        let { returnCode: returnCodeDeposit, depositNonce: depositNonceXYZ } = await interactor.deposit(firstUser, paymentXYZ);
        assert.isTrue(returnCodeDeposit.isSuccess());
>>>>>>> Fix Borrow, Withdraw, Deposit scenarios

        session.saveBreadcrumb("depositNonceXYZ", depositNonceXYZ)
    });

<<<<<<< HEAD
    it("withdraw token", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        
        await session.syncUsers([firstUser, secondUser]);

        let depositNonceOne = await session.loadBreadcrumb("depositNonceOne");
        let depositNonceTwo = await session.loadBreadcrumb("depositNonceTwo");
        let lendingAddress = await session.loadAddress("lendingAddr");
=======
    it("withdraw token XYZ", async function () {
        session.expectLongInteraction(this);
        await session.syncUsers([firstUser, secondUser]);
        let depositNonceXYZ = await session.loadBreadcrumb("depositNonceXYZ");

        let lendingAddress = await session.loadAddress("contractAddress");
>>>>>>> Fix Borrow, Withdraw, Deposit scenarios
        let lendingInteractor = await createLendingInteractor(session, lendingAddress);

        let tokenXYZ = await session.loadToken("tokenXYZ");
        let liquidityAddress = await lendingInteractor.getLiquidityAddress(tokenXYZ.identifier);
        let liquidityInteractor = await createLiquidityInteractor(session, liquidityAddress)
        let lendToken = await liquidityInteractor.getLendingToken();

        let paymentXYZ = TokenPayment.metaEsdtFromAmount(lendToken, depositNonceXYZ, 7, tokenXYZ.decimals)

        let returnCode = await lendingInteractor.withdraw(firstUser, paymentXYZ);
        assert.isTrue(returnCode.isSuccess());

    });

    it("borrow ABC token - collateral XYZ", async function () {
        session.expectLongInteraction(this);
        await session.syncUsers([firstUser, secondUser]);
        let tokenABC = await session.loadToken("tokenABC");
        let tokenXYZ = await session.loadToken("tokenXYZ");
        let depositNonceXYZ = await session.loadBreadcrumb("depositNonceXYZ");

        let lendingAddress = await session.loadAddress("contractAddress");
        let lendingInteractor = await createLendingInteractor(session, lendingAddress);

        let liquidityAddressABC = await lendingInteractor.getLiquidityAddress(tokenABC.identifier);
        let liquidityInteractorABC = await createLiquidityInteractor(session, liquidityAddressABC);
        await liquidityInteractorABC.getAggregatorAddress();
        await liquidityInteractorABC.getReserves();

        let liquidityAddressXYZ = await lendingInteractor.getLiquidityAddress(tokenXYZ.identifier);
        let liquidityInteractorXYZ = await createLiquidityInteractor(session, liquidityAddressXYZ);
        await liquidityInteractorXYZ.getAggregatorAddress();
        await liquidityInteractorXYZ.getReserves();

        let lendToken = await liquidityInteractorXYZ.getLendingToken();
        let assetToBorrow = await liquidityInteractorABC.getPoolAsset();

        await lendingInteractor.getAssetLoanToValue(tokenABC.identifier);
        await lendingInteractor.getAssetLoanToValue(tokenXYZ.identifier);

        let collateralPayment = TokenPayment.metaEsdtFromAmount(lendToken, depositNonceXYZ, 5, tokenXYZ.decimals)

        let { returnCode: returnBorrowCode, borrowNonce: returnBorrowNonce } = await lendingInteractor.borrow(firstUser, collateralPayment, assetToBorrow);
        assert.isTrue(returnBorrowCode.isSuccess());

    });

    it("generate report", async function () {
        await session.generateReport();

    it("destroy session", async function () {
        await session.destroy();
    });
});
