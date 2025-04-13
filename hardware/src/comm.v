`default_nettype none


/**
 * @brief The comm module is used to control the uart communication and
 * internal execution state machine.
 *
 * The protocol is defined as follows:
 * First, transmit a command:
 * - p: Ping, just sends back a pong (P)
 * - r: Read memory, sends back the full memory values
 * - w: Write memory, writes the value specified in the next 64 bytes to the ram
 * - x: Execute code, start at 0
 * - s: Get the status of the execution state machine (+/-)
 */
module comm (
    input clk,
    input [7:0] dataIn,
    input byteSending,
    output reg byteReady,
    output reg byteReadyOut,
    output reg [7:0] dataOut,

    output reg overrideMemControl,
    output reg overrideMemRnW,
    output reg [15:0] overrideMemAddr,
    output reg [15:0] overrideMemDataIn,
    input [15:0] overrideMemDataOut
);
    localparam COMM_STATE_IDLE = 0;
    localparam COMM_STATE_RXMEM = 2;
    localparam COMM_STATE_EXSTART = 3;
    localparam COMM_STATE_EXSTATE = 4;
    localparam COMM_STATE_TXMEM = 5;

    localparam MAX_MEMORY_COUNT = 32;

    initial begin
        byteReadyOut = 0;
        dataOut = 0;
        overrideMemControl = 0;
        overrideMemRnW = 0;
        overrideMemAddr = 0;
        overrideMemDataIn = 0;
    end

    reg [3:0] commState = COMM_STATE_IDLE;
    reg byteReceived = 0;
    reg [7:0] memCounter = 0;
    reg [7:0] lastRxByte = 0;
    reg hasReceivedLastByte = 0;

    always @(posedge clk) begin
        if (byteReady && !byteSending && !byteReceived) begin
            byteReceived <= 1'b1;

            case (commState)
                COMM_STATE_IDLE: begin
                    case (dataIn)
                        "p": begin
                            byteReadyOut <= 1'b1;
                            dataOut <= "P";
                        end
                        "r": begin
                            commState <= COMM_STATE_TXMEM;
                            memCounter <= 0;
                            byteReadyOut <= 1'b1;
                            dataOut <= "R";
                            hasReceivedLastByte <= 1'b0;
                        end
                        "w": begin
                            commState <= COMM_STATE_RXMEM;
                            memCounter <= 0;
                            byteReadyOut <= 1'b1;
                            hasReceivedLastByte <= 1'b0;
                            dataOut <= "W";
                        end
                        default: begin
                            byteReadyOut <= 1'b1;
                            dataOut <= "?";
                        end
                    endcase
                end

                COMM_STATE_TXMEM: begin
                    if (memCounter < MAX_MEMORY_COUNT) begin
                        byteReadyOut <= 1'b1;
                        overrideMemControl <= 1'b1;
                        overrideMemAddr <= memCounter;
                        overrideMemRnW <= 1'b1;
                        if (hasReceivedLastByte) begin
                            dataOut <= overrideMemDataOut[15:8];
                            memCounter <= memCounter + 1;
                            hasReceivedLastByte <= 1'b0;
                        end else begin
                            dataOut <= overrideMemDataOut[7:0];
                            hasReceivedLastByte <= 1'b1;
                        end
                    end else begin
                        commState <= COMM_STATE_IDLE;
                        byteReadyOut <= 1'b1;
                        overrideMemControl <= 1'b0;
                        dataOut <= "E";
                    end
                end
                COMM_STATE_RXMEM: begin
                    if (memCounter < MAX_MEMORY_COUNT) begin
                        if (hasReceivedLastByte) begin
                            byteReadyOut <= 1'b1;
                            overrideMemControl <= 1'b1;
                            overrideMemAddr <= memCounter;
                            overrideMemRnW <= 1'b0;
                            dataOut <= "-";
                            overrideMemDataIn <= {dataIn, lastRxByte};
                            memCounter <= memCounter + 1;
                            hasReceivedLastByte <= 1'b0;
                        end else begin
                            byteReadyOut <= 1'b1;
                            dataOut <= "+";
                            lastRxByte <= dataIn;
                            hasReceivedLastByte <= 1'b1;
                        end
                    end else begin
                        commState <= COMM_STATE_IDLE;
                        byteReadyOut <= 1'b1;
                        overrideMemControl <= 1'b0;
                        dataOut <= "E";
                    end
                end
                default: begin
                    commState <= COMM_STATE_IDLE;
                    byteReadyOut <= 1'b0;
                    dataOut <= 8'b0;
                end
            endcase
        end

        if (byteSending) begin
            byteReadyOut <= 1'b0;
            byteReceived <= 1'b0;
        end
    end

endmodule
