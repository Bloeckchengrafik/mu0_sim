module memory (
    input            memRq,
    input            readNotWrite,
    input      [7:0] addr,
    input      [7:0] dataIn,
    output reg [7:0] dataOut
);
    reg [15:0] memory[31:0];

    always @(*) begin
        if (memRq) begin
            if (readNotWrite) begin
                dataOut = memory[addr];
            end else begin
                memory[addr] = dataIn;
            end
        end else begin
            dataOut = 8'hFF;
        end
    end
endmodule
